use std::{marker::PhantomData, ops::Neg, ptr};

use rustc_hash::{FxHashMap, FxHashSet};

const MAX_TRIGRAMS: usize = 9;

pub struct NgramSearcher<'a> {
    max_word_count: usize,
    max_word_len: usize,
    max_query_len: usize,
    word_index: FxHashMap<String, FxHashSet<*const str>>,
    trigram_index: FxHashMap<[char; 3], FxHashSet<*const str>>,
    _phantom: PhantomData<&'a str>,
}

unsafe impl<'a> Send for NgramSearcher<'a> {}
unsafe impl<'a> Sync for NgramSearcher<'a> {}

const SEPARATORS: &[char] = &['_', '-', ' '];

impl<'a> NgramSearcher<'a> {
    pub fn new(items: &[&'a str]) -> Self {
        let mut word_index: FxHashMap<String, FxHashSet<*const str>> = FxHashMap::default();
        let mut trigram_index: FxHashMap<[char; 3], FxHashSet<*const str>> = FxHashMap::default();
        let mut max_word_len = 0;
        let mut max_query_len = 0;
        let mut max_words = 0;

        for &item in items {
            max_query_len = max_query_len.max(item.len());
            let mut word_count = 0;
            for word in item.split(SEPARATORS) {
                word_count += 1;
                if word.is_empty() {
                    continue;
                }

                max_word_len = max_word_len.max(item.len());

                word_index.entry(word.to_string()).or_default().insert(item);

                if word.len() >= 3 {
                    let chars = word.chars().collect::<Vec<_>>();
                    for window in chars.windows(3) {
                        trigram_index
                            .entry(unsafe { ptr::read(window.as_ptr() as *const [char; 3]) })
                            .or_default()
                            .insert(item);
                    }
                }
            }
            max_words = max_words.max(word_count);
        }

        Self {
            max_query_len: max_query_len + 6,
            max_word_len: max_word_len + 4,
            max_word_count: max_word_len + 2,
            word_index,
            trigram_index,
            _phantom: PhantomData,
        }
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<&'a str> {
        let query_lower = query.to_lowercase();
        let query_len = query_lower.len();

        if query.is_empty() || query_len > self.max_query_len {
            return vec![];
        }

        let words: FxHashSet<&str> = query_lower
            .split(SEPARATORS)
            .filter(|w| !w.is_empty() && w.len() <= self.max_word_len)
            .collect();

        if words.is_empty() || words.len() > self.max_word_count {
            return vec![];
        }

        let min_len = query_len.saturating_sub(3);

        let mut pool: Option<FxHashSet<*const str>> = None;
        let mut unknown_words = Vec::new();

        let mut words_to_intersect = vec![];
        for word in words {
            match self.word_index.get(word) {
                Some(items) => words_to_intersect.push(items),
                None => unknown_words.push(word),
            }
        }

        if !words_to_intersect.is_empty() {
            words_to_intersect.sort_unstable_by_key(|set| (set.len() as i64).neg());

            let mut intersect = words_to_intersect.pop().cloned().unwrap();

            for other_set in words_to_intersect.iter().rev() {
                intersect.retain(|ptr| other_set.contains(ptr));
                if intersect.is_empty() {
                    break;
                }
            }

            pool = Some(intersect);
        }
        let some_pool = pool.is_some();

        if some_pool && unknown_words.is_empty() {
            let mut results: Vec<_> = pool
                .unwrap()
                .into_iter()
                .map(|item| unsafe { &*item as &str })
                .collect();
            // Partial sort - only sort what we need
            if results.len() > limit {
                results.select_nth_unstable_by_key(limit, |item| item.len());
                results.truncate(limit);
            }
            results.sort_unstable_by_key(|item| item.len());
            return results;
        }

        // Score candidates
        let mut scores: FxHashMap<*const str, usize> = FxHashMap::default();
        scores.reserve(256);
        if let Some(pool) = &pool {
            for &item in pool {
                scores.insert(item, 1);
            }
        }
        let mut trigram_count = 0;
        'outer: for word in unknown_words {
            if word.len() < 3 || trigram_count >= MAX_TRIGRAMS {
                continue;
            }

            let mut chars = word.chars();
            let mut a = chars.next().unwrap();
            let mut b = chars.next().unwrap();

            for c in chars {
                if trigram_count >= MAX_TRIGRAMS {
                    break 'outer;
                }
                trigram_count += 1;

                let trigram = [a, b, c];

                let Some(items) = self.trigram_index.get(&trigram) else {
                    continue;
                };

                if some_pool {
                    for &item in items {
                        if let Some(score) = scores.get_mut(&item) {
                            *score += 1;
                        }
                    }
                } else {
                    for &item in items {
                        let len = unsafe { &*item }.len();
                        if len >= min_len {
                            *scores.entry(item).or_default() += 1;
                        }
                    }
                }

                // Slide window
                a = b;
                b = c;
            }
        }

        // Filter by minimum score
        let min_score = trigram_count.div_ceil(2);
        let mut results: Vec<_> = scores
            .into_iter()
            .filter(|(_, s)| *s >= min_score)
            .map(|(item, score)| (unsafe { &*item as &str }, score))
            .collect();

        if results.len() > limit {
            results.select_nth_unstable_by(limit, |a, b| {
                b.1.cmp(&a.1).then_with(|| a.0.len().cmp(&b.0.len()))
            });
            results.truncate(limit);
        }

        results.sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.len().cmp(&b.0.len())));

        results
            .into_iter()
            .take(limit)
            .map(|(item, _)| item)
            .collect()
    }
}
