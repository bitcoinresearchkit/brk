import { Computation } from './core';
import type { Effect, RenderEffect } from './effect';
export declare let Computations: Computation[], RenderEffects: RenderEffect[], Effects: Effect[];
/**
 * By default, changes are batched on the microtask queue which is an async process. You can flush
 * the queue synchronously to get the latest updates by calling `flushSync()`.
 */
export declare function flushSync(): void;
export declare function flushQueue(): void;
