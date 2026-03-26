const DEFAULT_SEPARATORS = "_- :/";
const DEFAULT_TRIGRAM_BUDGET = 6;
const DEFAULT_LIMIT = 100;
const DEFAULT_MIN_SCORE = 2;

/**
 * Search configuration.
 *
 * Defaults work well for most use cases.
 * Tweak `trigramBudget` to trade speed for typo tolerance.
 */
export class QuickMatchConfig {
  /** Characters that separate words in items (e.g. "hash_rate" → ["hash", "rate"]).
   * @type {string} */
  separators = DEFAULT_SEPARATORS;

  /** Max results returned per query.
   * @type {number} */
  limit = DEFAULT_LIMIT;

  /** How hard to try matching typos (0 = off, 3–6 = fast, 9–15 = thorough, max 20).
   * @type {number} */
  trigramBudget = DEFAULT_TRIGRAM_BUDGET;

  /** Min overlap required for a typo match. Higher = fewer false positives.
   * @type {number} */
  minScore = DEFAULT_MIN_SCORE;

  /** @param {number} n - Max results (default: 100, min: 1) */
  withLimit(n) {
    this.limit = Math.max(1, n);
    return this;
  }

  /** @param {number} n - Trigram budget (0-20, default: 6) */
  withTrigramBudget(n) {
    this.trigramBudget = Math.max(0, Math.min(20, n));
    return this;
  }

  /** @param {string} s - Separator characters (default: '_- :/') */
  withSeparators(s) {
    this.separators = s;
    return this;
  }

  /** @param {number} n - Min trigram score (default: 2, min: 1) */
  withMinScore(n) {
    this.minScore = Math.max(1, n);
    return this;
  }
}

/**
 * Instant search over a list of strings.
 *
 * Supports exact words, prefixes ("dom" → "dominance"), joined words
 * ("hashrate" → "hash_rate"), and typo tolerance ("suply" → "supply").
 * Results are ranked: exact matches first, then by specificity.
 */
export class QuickMatch {
  /** @param {string[]} items - Searchable items (lowercase) @param {QuickMatchConfig} [config] */
  constructor(items, config = new QuickMatchConfig()) {
    this.config = config;
    this.items = items;
    /** @type {Map<string, number[]>} */
    this.wordIndex = new Map();
    /** @type {Map<string, number[]>} */
    this.trigramIndex = new Map();
    this._sepLookup = sepLookup(config.separators);
    this._scores = new Uint32Array(items.length);
    /** @type {number[]} */
    this._dirty = [];

    let maxWordLen = 0;
    let maxQueryLen = 0;
    let maxWords = 0;
    const sep = this._sepLookup;

    for (let idx = 0; idx < items.length; idx++) {
      const item = items[idx];
      if (item.length > maxQueryLen) maxQueryLen = item.length;

      const words = [];
      let start = 0;

      for (let i = 0; i <= item.length; i++) {
        if (i < item.length && !sep[item.charCodeAt(i)]) continue;
        if (i > start) {
          const word = item.slice(start, i);
          words.push(word);
          if (word.length > maxWordLen) maxWordLen = word.length;
          for (let len = 1; len <= word.length; len++) {
            addToIndex(this.wordIndex, word.slice(0, len), idx);
          }
          for (let k = 0; k <= word.length - 3; k++) {
            addToIndex(this.trigramIndex, word[k] + word[k + 1] + word[k + 2], idx);
          }
        }
        start = i + 1;
      }

      for (let i = 0; i < words.length - 1; i++) {
        const compound = words[i] + words[i + 1];
        const from = words[i].length + 1;
        for (let len = from; len <= compound.length; len++) {
          addToIndex(this.wordIndex, compound.slice(0, len), idx);
        }
      }

      if (words.length > maxWords) maxWords = words.length;
    }

    this.maxWordLen = maxWordLen + 4;
    this.maxQueryLen = maxQueryLen + 6;
    this.maxWords = maxWords + 2;
  }

  /** @param {string} query */
  matches(query) {
    return this.matchesWith(query, this.config);
  }

  /**
   * @param {string} query
   * @param {QuickMatchConfig} config
   */
  matchesWith(query, config) {
    const { limit, trigramBudget } = config;
    const sep =
      config.separators === this.config.separators
        ? this._sepLookup
        : sepLookup(config.separators);

    const q = normalize(query);
    if (!q || q.length > this.maxQueryLen) return [];

    const qwords = splitWords(q, sep, this.maxWordLen);
    if (!qwords.length || qwords.length > this.maxWords) return [];

    const known = [];
    const unknown = [];

    for (const w of qwords) {
      const hits = this.wordIndex.get(w);
      if (hits) {
        known.push(hits);
      } else if (w.length >= 3 && unknown.length < trigramBudget) {
        unknown.push(w);
      }
    }

    const pool = intersect(known);

    // Try typo matching for unknown words
    if (unknown.length && trigramBudget) {
      const { _scores: scores, _dirty: dirty } = this;

      if (pool) {
        for (const i of pool) {
          scores[i] = 1;
          dirty.push(i);
        }
      }

      const hitCount = this._scoreTrigrams(
        unknown,
        trigramBudget,
        pool !== null,
        Math.max(0, q.length - 3),
      );
      const minScore = Math.max(config.minScore, Math.ceil(hitCount / 2));
      const result = this._rank(dirty, minScore, qwords, sep, limit);

      for (const i of dirty) scores[i] = 0;
      dirty.length = 0;

      if (result.length > 0) return result;
    }

    // Rank known candidates (intersection, or union as fallback)
    const candidates = pool || union(known);
    return candidates.length > 0
      ? this._rank(candidates, null, qwords, sep, limit)
      : [];
  }

  /** @private @param {string[]} unknown @param {number} budget @param {boolean} poolOnly @param {number} minLen */
  _scoreTrigrams(unknown, budget, poolOnly, minLen) {
    const { _scores: scores, _dirty: dirty, items } = this;
    const visited = new Set();
    const maxRounds = budget;
    let hits = 0;

    outer: for (let round = 0; round < maxRounds; round++) {
      for (const word of unknown) {
        if (budget <= 0) break outer;

        const pos = trigramPosition(word.length, round);
        if (pos < 0) continue;

        const tri = word[pos] + word[pos + 1] + word[pos + 2];
        if (visited.has(tri)) continue;
        visited.add(tri);
        budget--;

        const matched = this.trigramIndex.get(tri);
        if (!matched) continue;
        hits++;

        if (poolOnly) {
          for (let j = 0; j < matched.length; j++) {
            const i = matched[j];
            if (scores[i] > 0) scores[i]++;
          }
        } else {
          for (let j = 0; j < matched.length; j++) {
            const i = matched[j];
            if (items[i].length >= minLen) {
              if (scores[i] === 0) dirty.push(i);
              scores[i]++;
            }
          }
        }
      }
    }

    return hits;
  }

  /**
   * @private
   * @param {number[]} indices
   * @param {number|null} minScore
   * @param {string[]} qwords
   * @param {Uint8Array} sep
   * @param {number} limit
   */
  _rank(indices, minScore, qwords, sep, limit) {
    const { items, _scores: scores } = this;
    /** @type {[number[], number[], number[]]} */
    const buckets = [[], [], []]; // ps=0, ps=1, ps=2

    for (let i = 0; i < indices.length; i++) {
      const idx = indices[i];
      if (minScore !== null && scores[idx] < minScore) continue;
      buckets[prefixScore(items[idx], qwords, sep)].push(idx);
    }

    const results = [];
    for (let ps = 2; ps >= 0 && results.length < limit; ps--) {
      const bucket = buckets[ps];
      if (!bucket.length) continue;
      bucket.sort(
        (a, b) => scores[b] - scores[a] || items[a].length - items[b].length,
      );
      const take = Math.min(bucket.length, limit - results.length);
      for (let i = 0; i < take; i++) results.push(items[bucket[i]]);
    }

    return results;
  }
}

// --- Helpers ---

/** @param {string} query */
function normalize(query) {
  let out = "";
  let start = 0;
  let end = query.length;
  while (start < end && query.charCodeAt(start) <= 32) start++;
  while (end > start && query.charCodeAt(end - 1) <= 32) end--;
  for (let i = start; i < end; i++) {
    const c = query.charCodeAt(i);
    if (c >= 128) continue;
    out += c >= 65 && c <= 90 ? String.fromCharCode(c + 32) : query[i];
  }
  return out;
}

/** @param {string} separators */
function sepLookup(separators) {
  const t = new Uint8Array(128);
  for (let i = 0; i < separators.length; i++) {
    const c = separators.charCodeAt(i);
    if (c < 128) t[c] = 1;
  }
  return t;
}

/**
 * @param {string} text
 * @param {Uint8Array} sep
 * @param {number} maxLen
 */
function splitWords(text, sep, maxLen) {
  /** @type {string[]} */
  const words = [];
  let start = 0;
  for (let i = 0; i <= text.length; i++) {
    if (i < text.length && !sep[text.charCodeAt(i)]) continue;
    if (i > start) {
      const w = text.slice(start, i);
      if (w.length <= maxLen && !words.includes(w)) words.push(w);
    }
    start = i + 1;
  }
  return words;
}

/**
 * @param {Map<string, number[]>} index
 * @param {string} key
 * @param {number} value
 */
function addToIndex(index, key, value) {
  const arr = index.get(key);
  if (arr) {
    if (arr[arr.length - 1] !== value) arr.push(value);
  } else {
    index.set(key, [value]);
  }
}

/** @param {number[][]} arrays */
function union(arrays) {
  if (arrays.length <= 1) return arrays[0] || [];
  const seen = new Set();
  const result = [];
  for (const arr of arrays) {
    for (const idx of arr) {
      if (!seen.has(idx)) {
        seen.add(idx);
        result.push(idx);
      }
    }
  }
  return result;
}

/** @param {number[][]} arrays @returns {number[]|null} */
function intersect(arrays) {
  if (arrays.length <= 1) return arrays[0] || null;

  let si = 0;
  for (let i = 1; i < arrays.length; i++) {
    if (arrays[i].length < arrays[si].length) si = i;
  }

  const result = arrays[si].slice();
  for (let i = 0; i < arrays.length; i++) {
    if (i === si) continue;
    let w = 0;
    for (let j = 0; j < result.length; j++) {
      if (bsearch(arrays[i], result[j])) result[w++] = result[j];
    }
    result.length = w;
    if (!w) return null;
  }
  return result;
}

/**
 * @param {number[]} arr
 * @param {number} val
 */
function bsearch(arr, val) {
  let lo = 0,
    hi = arr.length - 1;
  while (lo <= hi) {
    const mid = (lo + hi) >> 1;
    if (arr[mid] === val) return true;
    if (arr[mid] < val) lo = mid + 1;
    else hi = mid - 1;
  }
  return false;
}

/** @param {string} item @param {string[]} qwords @param {Uint8Array} sep */
function prefixScore(item, qwords, sep) {
  let qi = 0,
    pos = 0;
  const len = item.length;

  while (qi < qwords.length) {
    while (pos < len && sep[item.charCodeAt(pos)]) pos++;
    if (pos >= len) return 0;

    const ws = pos;
    while (pos < len && !sep[item.charCodeAt(pos)]) pos++;

    const qw = qwords[qi];
    if (pos - ws < qw.length) return 0;
    for (let j = 0; j < qw.length; j++) {
      if (item.charCodeAt(ws + j) !== qw.charCodeAt(j)) return 0;
    }
    qi++;
  }

  while (pos < len && sep[item.charCodeAt(pos)]) pos++;
  return pos >= len ? 2 : 1;
}

/** @param {number} len @param {number} round */
function trigramPosition(len, round) {
  const max = len - 3;
  if (max < 0) return -1;
  if (round === 0) return 0;
  if (round === 1 && max > 0) return max;
  if (round === 2 && max > 1) return max >> 1;
  if (max <= 2) return -1;

  const mid = max >> 1;
  const off = (round - 2) >> 1;
  const pos = round & 1 ? Math.max(0, mid - off) : mid + off;
  return pos === 0 || pos >= max || pos === mid ? -1 : pos;
}
