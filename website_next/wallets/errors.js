/**
 * @param {unknown} error
 */
export function getErrorMessage(error) {
  return error instanceof Error ? error.message : "Request failed";
}
