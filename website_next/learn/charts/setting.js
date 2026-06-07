import { createRadioGroup } from "./radio.js";
import { createChartStorage } from "./storage.js";

/**
 * @template {string} T
 * @param {Object} config
 * @param {string} config.storageKey
 * @param {string} config.legend
 * @param {readonly { value: T, label: string }[]} config.options
 * @param {T} config.defaultValue
 */
export function createChartSetting(config) {
  const storage = createChartStorage(config.storageKey);

  return {
    /**
     * @param {string} chartKey
     * @param {T} [fallback]
     */
    get(chartKey, fallback = config.defaultValue) {
      const value = storage.get(chartKey);

      return (
        config.options.find((option) => option.value === value)?.value ??
        fallback
      );
    },

    /**
     * @param {string} chartKey
     * @param {T} value
     */
    save(chartKey, value) {
      storage.set(chartKey, value);
    },

    /**
     * @param {T} currentValue
     * @param {(value: T) => void} onChange
     */
    create(currentValue, onChange) {
      return createRadioGroup({
        legend: config.legend,
        options: config.options,
        currentValue,
        onChange,
      });
    },
  };
}
