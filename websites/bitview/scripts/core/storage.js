export default {
  /**
   * @param {string} key
   */
  readNumber(key) {
    const saved = this.read(key);
    if (saved) {
      return Number(saved);
    }
    return null;
  },
  /**
   * @param {string} key
   */
  readBool(key) {
    const saved = this.read(key);
    if (saved) {
      return saved === "true" || saved === "1";
    }
    return null;
  },
  /**
   * @param {string} key
   */
  read(key) {
    try {
      return localStorage.getItem(key);
    } catch (_) {
      return null;
    }
  },
  /**
   * @param {string} key
   * @param {string | boolean | null | undefined} value
   */
  write(key, value) {
    try {
      value !== undefined && value !== null
        ? localStorage.setItem(key, String(value))
        : localStorage.removeItem(key);
    } catch (_) {}
  },
  /**
   * @param {string} key
   */
  remove(key) {
    this.write(key, undefined);
  },
};
