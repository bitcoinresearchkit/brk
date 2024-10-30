import { Computation, type SignalOptions } from './core';
/**
 * Effects are the leaf nodes of our reactive graph. When their sources change, they are
 * automatically added to the queue of effects to re-execute, which will cause them to fetch their
 * sources and recompute
 */
export declare class BaseEffect<T = any> extends Computation<T> {
    _effect: (val: T, prev: T | undefined) => void;
    _modified: boolean;
    _prevValue: T | undefined;
    constructor(initialValue: T, compute: () => T, effect: (val: T, prev: T | undefined) => void, options?: SignalOptions<T>);
    write(value: T): T;
    _setError(error: unknown): void;
    _disposeNode(): void;
}
export declare class Effect<T = any> extends BaseEffect<T> {
    constructor(initialValue: T, compute: () => T, effect: (val: T, prev: T | undefined) => void, options?: SignalOptions<T>);
    _notify(state: number): void;
}
export declare class RenderEffect<T = any> extends BaseEffect<T> {
    constructor(initialValue: T, compute: () => T, effect: (val: T, prev: T | undefined) => void, options?: SignalOptions<T>);
    _notify(state: number): void;
}
