import type { SignalOptions } from "./core/index.js";
import { Owner } from "./core/index.js";
export interface Accessor<T> {
    (): T;
}
export interface Setter<T> {
    (value: T | SetValue<T>): T;
}
export interface SetValue<T> {
    (currentValue: T): T;
}
export type Signal<T> = [read: Accessor<T>, write: Setter<T>];
/**
 * Wraps the given value into a signal. The signal will return the current value when invoked
 * `fn()`, and provide a simple write API via `write()`. The value can now be observed
 * when used inside other computations created with `computed` and `effect`.
 */
export declare function createSignal<T>(initialValue: Exclude<T, Function>, options?: SignalOptions<T>): Signal<T>;
export declare function createSignal<T>(fn: (prev?: T) => T, initialValue?: T, options?: SignalOptions<T>): Signal<T>;
export declare function createAsync<T>(fn: (prev?: T) => Promise<T> | AsyncIterable<T> | T, initial?: T, options?: SignalOptions<T>): Accessor<T>;
/**
 * Creates a new computation whose value is computed and returned by the given function. The given
 * compute function is _only_ re-run when one of it's dependencies are updated. Dependencies are
 * are all signals that are read during execution.
 */
export declare function createMemo<T>(compute: (prev?: T) => T, initialValue?: T, options?: SignalOptions<T>): Accessor<T>;
/**
 * Invokes the given function each time any of the signals that are read inside are updated
 * (i.e., their value changes). The effect is immediately invoked on initialization.
 */
export declare function createEffect<T>(compute: () => T, effect: (v: T) => (() => void) | void, initialValue?: T, options?: {
    name?: string;
}): void;
/**
 * Invokes the given function each time any of the signals that are read inside are updated
 * (i.e., their value changes). The effect is immediately invoked on initialization.
 */
export declare function createRenderEffect<T>(compute: () => T, effect: (v: T) => (() => void) | void, initialValue?: T, options?: {
    name?: string;
}): void;
/**
 * Creates a computation root which is given a `dispose()` function to dispose of all inner
 * computations.
 */
export declare function createRoot<T>(init: ((dispose: () => void) => T) | (() => T)): T;
/**
 * Runs the given function in the given owner so that error handling and cleanups continue to work.
 *
 * Warning: Usually there are simpler ways of modeling a problem that avoid using this function
 */
export declare function runWithOwner<T>(owner: Owner | null, run: () => T): T | undefined;
/**
 * Runs the given function when an error is thrown in a child owner. If the error is thrown again
 * inside the error handler, it will trigger the next available parent owner handler.
 */
export declare function catchError<T>(fn: () => T, handler: (error: unknown) => void): void;
