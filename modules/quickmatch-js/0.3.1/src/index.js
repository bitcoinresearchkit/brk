const DEFAULT_SEPARATORS = "_- ";
const DEFAULT_TRIGRAM_BUDGET = 6;
const DEFAULT_LIMIT = 100;

/**
 * Configuration for QuickMatch.
 */
export class QuickMatchConfig {
  /** @type {string} Characters used to split items into words */
  separators = DEFAULT_SEPARATORS;

  /** @type {number} Maximum number of results to return */
  limit = DEFAULT_LIMIT;

  /** @type {number} Number of trigram lookups for fuzzy matching (0-20) */
  trigramBudget = DEFAULT_TRIGRAM_BUDGET;

  /**
   * Set maximum number of results.
   * @param {number} n
   */
  withLimit(n) {
    this.limit = Math.max(1, n);
    return this;
  }

  /**
   * Set trigram budget for fuzzy matching.
   * Higher values find more typos but cost more.
   * @param {number} n - Budget (0-20, default: 6)
   */
  withTrigramBudget(n) {
    this.trigramBudget = Math.max(0, Math.min(20, n));
    return this;
  }

  /**
   * Set word separator characters.
   * @param {string} s - Separator characters (default: '_- ')
   */
  withSeparators(s) {
    this.separators = s;
    return this;
  }
}

/**
 * Fast fuzzy string matcher using word and trigram indexing.
 */
export class QuickMatch {
  /**
   * Create a new matcher.
   * @param {string[]} items - Items to index (should be lowercase)
   * @param {QuickMatchConfig} [config] - Optional configuration
   */
  constructor(items, config = new QuickMatchConfig()) {
    this.config = config;
    this.items = items;
    /** @type {Map<string, number[]>} */
    this.wordIndex = new Map();
    /** @type {Map<string, number[]>} */
    this.trigramIndex = new Map();

    let maxWordLength = 0;
    let maxQueryLength = 0;
    let maxWordCount = 0;

    const { separators } = config;

    for (let itemIndex = 0; itemIndex < items.length; itemIndex++) {
      const item = items[itemIndex];

      if (item.length > maxQueryLength) {
        maxQueryLength = item.length;
      }

      let wordCount = 0;
      let wordStart = 0;

      for (let i = 0; i <= item.length; i++) {
        const isEndOfWord = i === item.length || separators.includes(item[i]);

        if (isEndOfWord && i > wordStart) {
          wordCount++;
          const word = item.slice(wordStart, i);

          if (word.length > maxWordLength) {
            maxWordLength = word.length;
          }

          addToIndex(this.wordIndex, word, itemIndex);
          addTrigramsToIndex(this.trigramIndex, word, itemIndex);

          wordStart = i + 1;
        } else if (isEndOfWord) {
          wordStart = i + 1;
        }
      }

      if (wordCount > maxWordCount) {
        maxWordCount = wordCount;
      }
    }

    this.maxWordLength = maxWordLength + 4;
    this.maxQueryLength = maxQueryLength + 6;
    this.maxWordCount = maxWordCount + 2;
  }

  /**
   * Find matching items. Returns items sorted by relevance.
   * @param {string} query - Search query
   */
  matches(query) {
    return this.matchesWith(query, this.config);
  }

  /**
   * Find matching items with custom config. Returns items sorted by relevance.
   * @param {string} query - Search query
   * @param {QuickMatchConfig} config - Configuration to use
   */
  matchesWith(query, config) {
    const { limit, trigramBudget, separators } = config;

    const normalizedQuery = normalizeQuery(query);

    if (!normalizedQuery || normalizedQuery.length > this.maxQueryLength) {
      return [];
    }

    const queryWords = parseWords(
      normalizedQuery,
      separators,
      this.maxWordLength,
    );

    if (!queryWords.length || queryWords.length > this.maxWordCount) {
      return [];
    }

    const knownWords = [];
    const unknownWords = [];

    for (const word of queryWords) {
      const matchingItems = this.wordIndex.get(word);

      if (matchingItems) {
        knownWords.push(matchingItems);
      } else if (word.length >= 3 && unknownWords.length < trigramBudget) {
        unknownWords.push(word);
      }
    }

    const exactMatches = intersectAll(knownWords);
    const hasExactMatches = exactMatches.length > 0;
    const needsFuzzyMatching = unknownWords.length > 0 && trigramBudget > 0;

    if (!needsFuzzyMatching) {
      if (!hasExactMatches) return [];
      return this.sortedByLength(exactMatches, limit);
    }

    const scores = new Map();

    if (hasExactMatches) {
      for (const index of exactMatches) {
        scores.set(index, 1);
      }
    }

    const minItemLength = Math.max(0, normalizedQuery.length - 3);

    const hitCount = this.scoreByTrigrams({
      unknownWords,
      budget: trigramBudget,
      scores,
      hasExactMatches,
      minItemLength,
    });

    const minScoreToInclude = Math.max(1, Math.ceil(hitCount / 2));

    return this.rankedResults(scores, minScoreToInclude, limit);
  }

  /**
   * @private
   * @param {{unknownWords: string[], budget: number, scores: Map<number, number>, hasExactMatches: boolean, minItemLength: number}} args
   */
  scoreByTrigrams({
    unknownWords,
    budget,
    scores,
    hasExactMatches,
    minItemLength,
  }) {
    const visitedTrigrams = new Set();
    let budgetRemaining = budget;
    let hitCount = 0;

    outer: for (let round = 0; round < budget; round++) {
      for (const word of unknownWords) {
        if (budgetRemaining <= 0) break outer;

        const position = pickTrigramPosition(word.length, round);
        if (position < 0) continue;

        const trigram =
          word[position] + word[position + 1] + word[position + 2];

        if (visitedTrigrams.has(trigram)) continue;
        visitedTrigrams.add(trigram);

        budgetRemaining--;

        const matchingItems = this.trigramIndex.get(trigram);
        if (!matchingItems) continue;

        hitCount++;

        for (const itemIndex of matchingItems) {
          if (hasExactMatches) {
            const currentScore = scores.get(itemIndex);
            if (currentScore !== undefined) {
              scores.set(itemIndex, currentScore + 1);
            }
          } else if (this.items[itemIndex].length >= minItemLength) {
            scores.set(itemIndex, (scores.get(itemIndex) || 0) + 1);
          }
        }
      }
    }

    return hitCount;
  }

  /**
   * @private
   * @param {number[]} indices
   * @param {number} limit
   */
  sortedByLength(indices, limit) {
    const { items } = this;
    indices.sort((a, b) => items[a].length - items[b].length);
    if (indices.length > limit) indices.length = limit;
    return indices.map((i) => items[i]);
  }

  /**
   * @private
   * @param {Map<number, number>} scores
   * @param {number} minScore
   * @param {number} limit
   */
  rankedResults(scores, minScore, limit) {
    const { items } = this;
    const results = [];

    for (const [index, score] of scores) {
      if (score >= minScore) {
        results.push({ index, score });
      }
    }

    results.sort((a, b) => {
      if (b.score !== a.score) return b.score - a.score;
      return items[a.index].length - items[b.index].length;
    });

    if (results.length > limit) results.length = limit;

    return results.map((r) => items[r.index]);
  }
}

/** @param {string} query */
function normalizeQuery(query) {
  let result = "";
  let start = 0;
  let end = query.length;

  while (start < end && query.charCodeAt(start) <= 32) start++;
  while (end > start && query.charCodeAt(end - 1) <= 32) end--;

  for (let i = start; i < end; i++) {
    const code = query.charCodeAt(i);
    if (code >= 128) continue;
    result +=
      code >= 65 && code <= 90 ? String.fromCharCode(code + 32) : query[i];
  }

  return result;
}

/**
 * @param {string} text
 * @param {string} separators
 * @param {number} maxLength
 */
function parseWords(text, separators, maxLength) {
  /** @type {string[]} */
  const words = [];
  let start = 0;

  for (let i = 0; i <= text.length; i++) {
    const isEnd = i === text.length || separators.includes(text[i]);

    if (isEnd && i > start) {
      const word = text.slice(start, i);
      if (word.length <= maxLength && !words.includes(word)) {
        words.push(word);
      }
      start = i + 1;
    } else if (isEnd) {
      start = i + 1;
    }
  }

  return words;
}

/**
 * @param {Map<string, number[]>} index
 * @param {string} key
 * @param {number} value
 */
function addToIndex(index, key, value) {
  const existing = index.get(key);
  if (existing) {
    existing.push(value);
  } else {
    index.set(key, [value]);
  }
}

/**
 * @param {Map<string, number[]>} index
 * @param {string} word
 * @param {number} itemIndex
 */
function addTrigramsToIndex(index, word, itemIndex) {
  if (word.length < 3) return;

  for (let i = 0; i <= word.length - 3; i++) {
    const trigram = word[i] + word[i + 1] + word[i + 2];
    const existing = index.get(trigram);

    if (!existing) {
      index.set(trigram, [itemIndex]);
    } else if (existing[existing.length - 1] !== itemIndex) {
      existing.push(itemIndex);
    }
  }
}

/** @param {number[][]} arrays */
function intersectAll(arrays) {
  if (!arrays.length) return [];

  let smallestIndex = 0;
  for (let i = 1; i < arrays.length; i++) {
    if (arrays[i].length < arrays[smallestIndex].length) {
      smallestIndex = i;
    }
  }

  const result = arrays[smallestIndex].slice();

  for (let i = 0; i < arrays.length && result.length > 0; i++) {
    if (i === smallestIndex) continue;

    let writeIndex = 0;
    for (let j = 0; j < result.length; j++) {
      if (binarySearch(arrays[i], result[j])) {
        result[writeIndex++] = result[j];
      }
    }
    result.length = writeIndex;
  }

  return result;
}

/**
 * @param {number[]} sortedArray
 * @param {number} value
 */
function binarySearch(sortedArray, value) {
  let low = 0;
  let high = sortedArray.length - 1;

  while (low <= high) {
    const mid = (low + high) >> 1;
    const midValue = sortedArray[mid];

    if (midValue === value) return true;
    if (midValue < value) low = mid + 1;
    else high = mid - 1;
  }

  return false;
}

/**
 * @param {number} wordLength
 * @param {number} round
 */
function pickTrigramPosition(wordLength, round) {
  const maxPosition = wordLength - 3;
  if (maxPosition < 0) return -1;

  if (round === 0) return 0;
  if (round === 1 && maxPosition > 0) return maxPosition;
  if (round === 2 && maxPosition > 1) return maxPosition >> 1;
  if (maxPosition <= 2) return -1;

  const middle = maxPosition >> 1;
  const offset = (round - 2) >> 1;
  const position = round & 1 ? Math.max(0, middle - offset) : middle + offset;

  if (position === 0 || position >= maxPosition || position === middle) {
    return -1;
  }

  return position;
}
