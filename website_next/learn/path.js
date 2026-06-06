import { createId } from "../utils/id.js";

/** @param {readonly string[]} path */
export function createPathId(path) {
  return createId(path.join(" "));
}
