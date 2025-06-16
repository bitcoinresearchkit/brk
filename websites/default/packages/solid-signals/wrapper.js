// @ts-check

/**
 * @import { SignalOptions } from "./v0.3.2/types/core/core"
 * @import { getOwner as GetOwner, onCleanup as OnCleanup } from "./v0.3.2/types/core/owner"
 * @import { createSignal as CreateSignal, createEffect as CreateEffect, createMemo as CreateMemo, createRoot as CreateRoot, runWithOwner as RunWithOwner, Setter } from "./v0.3.2/types/signals";
 */

/**
 * @template T
 * @typedef {() => T} Accessor
 */

/**
 * @template T
 * @typedef {Accessor<T> & { set: Setter<T>; reset: VoidFunction }} Signal
 */

import {
  createSignal,
  createEffect,
  getOwner,
  createMemo,
  createRoot,
  runWithOwner,
  onCleanup,
} from "./v0.3.2/script.js";

let effectCount = 0;

const signals = {
  createSolidSignal: /** @type {typeof CreateSignal} */ (createSignal),
  createSolidEffect: /** @type {typeof CreateEffect} */ (createEffect),
  createEffect: /** @type {typeof CreateEffect} */ (
    // @ts-ignore
    (compute, effect) => {
      let dispose = /** @type {VoidFunction | null} */ (null);

      if (getOwner() === null) {
        throw Error("No owner");
      }

      function cleanup() {
        if (dispose) {
          dispose();
          dispose = null;
          // console.log("effectCount = ", --effectCount);
        }
      }

      // @ts-ignore
      createEffect(compute, (v, oldV) => {
        // console.log("effectCount = ", ++effectCount);
        cleanup();
        signals.createRoot((_dispose) => {
          dispose = _dispose;
          return effect(v, oldV);
        });
        signals.onCleanup(cleanup);
      });
      signals.onCleanup(cleanup);
    }
  ),
  createMemo: /** @type {typeof CreateMemo} */ (createMemo),
  createRoot: /** @type {typeof CreateRoot} */ (createRoot),
  getOwner: /** @type {typeof GetOwner} */ (getOwner),
  runWithOwner: /** @type {typeof RunWithOwner} */ (runWithOwner),
  onCleanup: /** @type {typeof OnCleanup} */ (onCleanup),
  /**
   * @template T
   * @param {T} initialValue
   * @param {SignalOptions<T> & {save?: {keyPrefix: string | Accessor<string>; key: string; serialize: (v: T) => string; deserialize: (v: string) => T; serializeParam?: boolean; saveDefaultValue?: boolean}}} [options]
   * @returns {Signal<T>}
   */
  createSignal(initialValue, options) {
    const [get, set] = this.createSolidSignal(
      /** @type {any} */ (initialValue),
      options
    );

    // @ts-ignore
    get.set = set;

    // @ts-ignore
    get.reset = () => set(initialValue);

    if (options?.save) {
      const save = options.save;

      const paramKey = save.key;
      const storageKey = this.createMemo(
        () =>
          `${
            typeof save.keyPrefix === "string"
              ? save.keyPrefix
              : save.keyPrefix()
          }-${paramKey}`
      );

      let serialized = /** @type {string | null} */ (null);
      if (options.save.serializeParam !== false) {
        serialized = new URLSearchParams(window.location.search).get(paramKey);
      }
      if (serialized === null) {
        try {
          serialized = localStorage.getItem(storageKey());
        } catch (_) {}
      }
      if (serialized) {
        set(() => (serialized ? save.deserialize(serialized) : initialValue));
      }

      let firstRun1 = true;
      this.createEffect(storageKey, (storageKey) => {
        if (!firstRun1) {
          try {
            serialized = localStorage.getItem(storageKey);
            set(() =>
              serialized ? save.deserialize(serialized) : initialValue
            );
          } catch (_) {}
        }
        firstRun1 = false;
      });

      let firstRun2 = true;
      this.createEffect(get, (value) => {
        if (!save) return;

        if (!firstRun2) {
          try {
            if (
              value !== undefined &&
              value !== null &&
              (initialValue === undefined ||
                initialValue === null ||
                save.saveDefaultValue ||
                save.serialize(value) !== save.serialize(initialValue))
            ) {
              localStorage.setItem(storageKey(), save.serialize(value));
            } else {
              localStorage.removeItem(storageKey());
            }
          } catch (_) {}
        }

        if (
          value !== undefined &&
          value !== null &&
          (initialValue === undefined ||
            initialValue === null ||
            save.saveDefaultValue ||
            save.serialize(value) !== save.serialize(initialValue))
        ) {
          writeParam(paramKey, save.serialize(value));
        } else {
          removeParam(paramKey);
        }

        firstRun2 = false;
      });
    }

    // @ts-ignore
    return get;
  },
};
/** @typedef {typeof signals} Signals */

/**
 * @param {string} key
 * @param {string | undefined} value
 */
function writeParam(key, value) {
  const urlParams = new URLSearchParams(window.location.search);

  if (value !== undefined) {
    urlParams.set(key, String(value));
  } else {
    urlParams.delete(key);
  }

  try {
    window.history.replaceState(
      null,
      "",
      `${window.location.pathname}?${urlParams.toString()}`
    );
  } catch (_) {}
}

/**
 * @param {string} key
 */
function removeParam(key) {
  writeParam(key, undefined);
}

export default signals;
