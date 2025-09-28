import { INDEX_TO_WORD } from "./generated/metrics";

/** @type {Record<string, number>} */
const WORD_TO_INDEX = {};

INDEX_TO_WORD.forEach((word, index) => {
  WORD_TO_INDEX[word] = index;
});

/**
 * @param {string} metric
 */
function compressMetric(metric) {
  return metric
    .split("_")
    .map((word) => {
      const index = WORD_TO_INDEX[word];
      return index !== undefined ? indexToLetters(index) : word;
    })
    .join("_");
}

/**
 * @param {string} compressedMetric
 */
function decompressMetric(compressedMetric) {
  return compressedMetric
    .split("_")
    .map((code) => {
      const index = lettersToIndex(code);
      return INDEX_TO_WORD[index] || code; // Fallback to original if not found
    })
    .join("_");
}

/**
 * @param {string} letters
 */
function lettersToIndex(letters) {
  let result = 0;
  for (let i = 0; i < letters.length; i++) {
    const value = charToIndex(letters.charCodeAt(i));
    result = result * 52 + value + 1;
  }
  return result - 1;
}

/**
 * @param {number} byte
 */
function charToIndex(byte) {
  if (byte >= 65 && byte <= 90) {
    // 'A' to 'Z'
    return byte - 65;
  } else if (byte >= 97 && byte <= 122) {
    // 'a' to 'z'
    return byte - 97 + 26;
  } else {
    return 255; // Invalid
  }
}

/**
 * @param {number} index
 */
function indexToLetters(index) {
  if (index < 52) {
    return indexToChar(index);
  }
  let result = [];
  while (true) {
    result.push(indexToChar(index % 52));
    index = Math.floor(index / 52);
    if (index === 0) break;
    index -= 1;
  }
  return result.reverse().join("");
}

/**
 * @param {number} index
 */
function indexToChar(index) {
  if (index <= 25) {
    return String.fromCharCode(65 + index); // A-Z
  } else {
    return String.fromCharCode(97 + index - 26); // a-z
  }
}
