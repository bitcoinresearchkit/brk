/** Market utilities */

/**
 * Convert period ID to readable name
 * @param {string} id
 * @param {boolean} [compoundAdjective]
 */
export function periodIdToName(id, compoundAdjective) {
  const num = parseInt(id);
  const s = compoundAdjective || num === 1 ? "" : "s";
  switch (id.slice(-1)) {
    case "h":
      return `${num} hour${s}`;
    case "d":
      return `${num} day${s}`;
    case "w":
      return `${num} week${s}`;
    case "m":
      return `${num} month${s}`;
    case "y":
      return `${num} year${s}`;
    default:
      return id;
  }
}
