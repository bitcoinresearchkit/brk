/** @param {string} name */
export function createChartStorage(name) {
  const prefix = `bitview:chart-${name}`;

  return {
    /** @param {string} chartKey */
    get(chartKey) {
      return localStorage.getItem(`${prefix}:${chartKey}`);
    },

    /**
     * @param {string} chartKey
     * @param {string} value
     */
    set(chartKey, value) {
      localStorage.setItem(`${prefix}:${chartKey}`, value);
    },
  };
}
