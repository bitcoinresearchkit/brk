import type { Effect } from "./effect.js";
import { Computation } from "./core.js";
export declare let Computations: Computation[], RenderEffects: Effect[], Effects: Effect[];
/**
 * By default, changes are batched on the microtask queue which is an async process. You can flush
 * the queue synchronously to get the latest updates by calling `flushSync()`.
 */
export declare function flushSync(): void;
export declare function flushQueue(): void;
