/**
 * @import { SignalOptions } from "./modules/solidjs-signals/0.6.3/dist/types/core/core.js"
 * @import { getOwner as GetOwner, onCleanup as OnCleanup } from "./modules/solidjs-signals/0.6.3/dist/types/core/owner.js"
 * @import { createSignal as CreateSignal, createEffect as CreateEffect, createMemo as CreateMemo, createRoot as CreateRoot, runWithOwner as RunWithOwner, Setter } from "./modules/solidjs-signals/0.6.3/dist/types/signals.js";
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
} from "./modules/solidjs-signals/0.6.3/dist/prod.js";
import { createPersistedValue } from "./utils/persisted.js";

// let effectCount = 0;

const signals = {
  createSolidSignal: /** @type {typeof CreateSignal} */ (createSignal),
  createEffect: /** @type {typeof CreateEffect} */ (createEffect),
  createScopedEffect: /** @type {typeof CreateEffect} */ (
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
   * @param {SignalOptions<T>} [options]
   * @returns {Signal<T>}
   */
  createSignal(initialValue, options) {
    const [get, set] = this.createSolidSignal(
      /** @type {any} */ (initialValue),
      options,
    );

    // @ts-ignore
    get.set = set;

    // @ts-ignore
    get.reset = () => set(initialValue);

    // @ts-ignore
    return get;
  },
  /**
   * @template T
   * @param {Object} args
   * @param {T} args.defaultValue
   * @param {string} args.storageKey
   * @param {string} [args.urlKey]
   * @param {(v: T) => string} args.serialize
   * @param {(s: string) => T} args.deserialize
   * @param {boolean} [args.saveDefaultValue]
   * @returns {Signal<T>}
   */
  createPersistedSignal({
    defaultValue,
    storageKey,
    urlKey,
    serialize,
    deserialize,
    saveDefaultValue = false,
  }) {
    const persisted = createPersistedValue({
      defaultValue,
      storageKey,
      urlKey,
      serialize,
      deserialize,
      saveDefaultValue,
    });

    const signal = this.createSignal(persisted.value);

    // Sync signal changes to persisted storage
    let firstRun = true;
    this.createEffect(signal, (value) => {
      if (firstRun) {
        firstRun = false;
      } else {
        persisted.set(value);
      }
    });

    return signal;
  },
};
/** @typedef {typeof signals} Signals */

export default signals;
