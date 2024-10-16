import { Computation, type MemoOptions } from './core';
/**
 * By default, changes are batched on the microtask queue which is an async process. You can flush
 * the queue synchronously to get the latest updates by calling `flushSync()`.
 */
export declare function flushSync(): void;
/**
 * Effects are the leaf nodes of our reactive graph. When their sources change, they are
 * automatically added to the queue of effects to re-execute, which will cause them to fetch their
 * sources and recompute
 */
export declare class Effect<T = any> extends Computation<T> {
    constructor(initialValue: T, compute: () => T, options?: MemoOptions<T>);
    _notify(state: number): void;
    write(value: T): T;
    _setError(error: unknown): void;
}
export declare class RenderEffect<T = any> extends Computation<T> {
    effect: (val: T) => void;
    modified: boolean;
    constructor(initialValue: T, compute: () => T, effect: (val: T) => void, options?: MemoOptions<T>);
    _notify(state: number): void;
    write(value: T): T;
    _setError(error: unknown): void;
}
