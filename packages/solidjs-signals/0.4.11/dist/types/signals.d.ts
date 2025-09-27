import type { SignalOptions } from "./core/index.js";
import { Owner } from "./core/index.js";
export type Accessor<T> = () => T;
export type Setter<in out T> = {
    <U extends T>(...args: undefined extends T ? [] : [value: Exclude<U, Function> | ((prev: T) => U)]): undefined extends T ? undefined : U;
    <U extends T>(value: (prev: T) => U): U;
    <U extends T>(value: Exclude<U, Function>): U;
    <U extends T>(value: Exclude<U, Function> | ((prev: T) => U)): U;
};
export type Signal<T> = [get: Accessor<T>, set: Setter<T>];
export type ComputeFunction<Prev, Next extends Prev = Prev> = (v: Prev) => Next;
export type EffectFunction<Prev, Next extends Prev = Prev> = (v: Next, p?: Prev) => (() => void) | void;
export type EffectBundle<Prev, Next extends Prev = Prev> = {
    effect: EffectFunction<Prev, Next>;
    error: (err: unknown, cleanup: () => void) => void;
};
export interface EffectOptions {
    name?: string;
    defer?: boolean;
}
export interface MemoOptions<T> {
    name?: string;
    equals?: false | ((prev: T, next: T) => boolean);
}
export type NoInfer<T extends any> = [T][T extends any ? 0 : never];
/**
 * Creates a simple reactive state with a getter and setter
 * ```typescript
 * const [state: Accessor<T>, setState: Setter<T>] = createSignal<T>(
 *  value: T,
 *  options?: { name?: string, equals?: false | ((prev: T, next: T) => boolean) }
 * )
 * ```
 * @param value initial value of the state; if empty, the state's type will automatically extended with undefined; otherwise you need to extend the type manually if you want setting to undefined not be an error
 * @param options optional object with a name for debugging purposes and equals, a comparator function for the previous and next value to allow fine-grained control over the reactivity
 *
 * @returns ```typescript
 * [state: Accessor<T>, setState: Setter<T>]
 * ```
 * * the Accessor is a function that returns the current value and registers each call to the reactive root
 * * the Setter is a function that allows directly setting or mutating the value:
 * ```typescript
 * const [count, setCount] = createSignal(0);
 * setCount(count => count + 1);
 * ```
 *
 * @description https://docs.solidjs.com/reference/basic-reactivity/create-signal
 */
export declare function createSignal<T>(): Signal<T | undefined>;
export declare function createSignal<T>(value: Exclude<T, Function>, options?: SignalOptions<T>): Signal<T>;
export declare function createSignal<T>(fn: ComputeFunction<T>, initialValue?: T, options?: SignalOptions<T>): Signal<T>;
/**
 * Creates a readonly derived reactive memoized signal
 * ```typescript
 * export function createMemo<T>(
 *   compute: (v: T) => T,
 *   value?: T,
 *   options?: { name?: string, equals?: false | ((prev: T, next: T) => boolean) }
 * ): () => T;
 * ```
 * @param compute a function that receives its previous or the initial value, if set, and returns a new value used to react on a computation
 * @param value an optional initial value for the computation; if set, fn will never receive undefined as first argument
 * @param options allows to set a name in dev mode for debugging purposes and use a custom comparison function in equals
 *
 * @description https://docs.solidjs.com/reference/basic-reactivity/create-memo
 */
export declare function createMemo<Next extends Prev, Prev = Next>(compute: ComputeFunction<undefined | NoInfer<Prev>, Next>): Accessor<Next>;
export declare function createMemo<Next extends Prev, Init = Next, Prev = Next>(compute: ComputeFunction<Init | Prev, Next>, value: Init, options?: MemoOptions<Next>): Accessor<Next>;
/**
 * Creates a readonly derived async reactive memoized signal
 * ```typescript
 * export function createAsync<T>(
 *   compute: (v: T) => Promise<T> | T,
 *   value?: T,
 *   options?: { name?: string, equals?: false | ((prev: T, next: T) => boolean) }
 * ): () => T;
 * ```
 * @param compute a function that receives its previous or the initial value, if set, and returns a new value used to react on a computation
 * @param value an optional initial value for the computation; if set, fn will never receive undefined as first argument
 * @param options allows to set a name in dev mode for debugging purposes and use a custom comparison function in equals
 *
 * @description https://docs.solidjs.com/reference/basic-reactivity/create-async
 */
export declare function createAsync<T>(compute: (prev: T | undefined, refreshing: boolean) => Promise<T> | AsyncIterable<T> | T, value?: T, options?: MemoOptions<T>): Accessor<T> & {
    refresh: () => void;
};
/**
 * Creates a reactive effect that runs after the render phase
 * ```typescript
 * export function createEffect<T>(
 *   compute: (prev: T) => T,
 *   effect: (v: T, prev: T) => (() => void) | void,
 *   value?: T,
 *   options?: { name?: string }
 * ): void;
 * ```
 * @param compute a function that receives its previous or the initial value, if set, and returns a new value used to react on a computation
 * @param effect a function that receives the new value and is used to perform side effects, return a cleanup function to run on disposal
 * @param error an optional function that receives an error if thrown during the computation
 * @param value an optional initial value for the computation; if set, fn will never receive undefined as first argument
 * @param options allows to set a name in dev mode for debugging purposes
 *
 * @description https://docs.solidjs.com/reference/basic-reactivity/create-effect
 */
export declare function createEffect<Next>(compute: ComputeFunction<undefined | NoInfer<Next>, Next>, effect: EffectFunction<NoInfer<Next>, Next> | EffectBundle<NoInfer<Next>, Next>): void;
export declare function createEffect<Next, Init = Next>(compute: ComputeFunction<Init | Next, Next>, effect: EffectFunction<Next, Next> | EffectBundle<Next, Next>, value: Init, options?: EffectOptions): void;
/**
 * Creates a reactive computation that runs during the render phase as DOM elements are created and updated but not necessarily connected
 * ```typescript
 * export function createRenderEffect<T>(
 *   compute: (prev: T) => T,
 *   effect: (v: T, prev: T) => (() => void) | void,
 *   value?: T,
 *   options?: { name?: string }
 * ): void;
 * ```
 * @param compute a function that receives its previous or the initial value, if set, and returns a new value used to react on a computation
 * @param effect a function that receives the new value and is used to perform side effects
 * @param value an optional initial value for the computation; if set, fn will never receive undefined as first argument
 * @param options allows to set a name in dev mode for debugging purposes
 *
 * @description https://docs.solidjs.com/reference/secondary-primitives/create-render-effect
 */
export declare function createRenderEffect<Next>(compute: ComputeFunction<undefined | NoInfer<Next>, Next>, effect: EffectFunction<NoInfer<Next>, Next>): void;
export declare function createRenderEffect<Next, Init = Next>(compute: ComputeFunction<Init | Next, Next>, effect: EffectFunction<Next, Next>, value: Init, options?: EffectOptions): void;
/**
 * Creates a new non-tracked reactive context with manual disposal
 *
 * @param fn a function in which the reactive state is scoped
 * @returns the output of `fn`.
 *
 * @description https://docs.solidjs.com/reference/reactive-utilities/create-root
 */
export declare function createRoot<T>(init: ((dispose: () => void) => T) | (() => T), options?: {
    id: string;
}): T;
/**
 * Runs the given function in the given owner to move ownership of nested primitives and cleanups.
 * This method untracks the current scope.
 *
 * Warning: Usually there are simpler ways of modeling a problem that avoid using this function
 */
export declare function runWithOwner<T>(owner: Owner | null, run: () => T): T;
/**
 * Returns a promise of the resolved value of a reactive expression
 * @param fn a reactive expression to resolve
 */
export declare function resolve<T>(fn: () => T): Promise<T>;
/**
 * Runs the given function and returns a tuple with the result or an error.
 * If the function throws an error, it will be caught and returned as the first element of the tuple.
 * If the function returns a promise, it will resolve to a tuple with the result or an error.
 *
 * @param fn The function to run.
 * @returns A tuple with either [undefined, result] or [error].
 *
 * @description https://docs.solidjs.com/reference/reactive-utilities/try-catch
 */
export type TryCatchResult<T, E> = [undefined, T] | [E];
export declare function tryCatch<T, E = Error>(fn: () => Promise<T>): Promise<TryCatchResult<T, E>>;
export declare function tryCatch<T, E = Error>(fn: () => T): TryCatchResult<T, E>;
/**
 * Creates an optimistic signal that can be used to optimistically update a value
 * and then revert it back to the previous value at end of transition.
 *
 * @param initial The initial value of the signal.
 * @param compute An optional function to compute the next value based on the previous value and change.
 * @param options Optional signal options.
 *
 * @returns A tuple containing an accessor for the current value and a setter function to apply changes.
 */
export declare function createOptimistic<T>(initial: Exclude<T, Function>, compute?: never): [Accessor<T>, (value: T | ((v?: T) => T)) => void];
export declare function createOptimistic<T extends object, U>(initial: T | Accessor<T>, compute: (prev: T, change: U) => void, key: string | ((item: any) => any)): [Accessor<T>, (value: U | ((v?: U) => U)) => void];
