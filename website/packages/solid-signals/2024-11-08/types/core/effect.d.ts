import { Computation, type SignalOptions } from "./core.js";
/**
 * Effects are the leaf nodes of our reactive graph. When their sources change, they are
 * automatically added to the queue of effects to re-execute, which will cause them to fetch their
 * sources and recompute
 */
export declare class Effect<T = any> extends Computation<T> {
    _effect: (val: T, prev: T | undefined) => void;
    _modified: boolean;
    _prevValue: T | undefined;
    _type: 0 | 1;
    constructor(initialValue: T, compute: () => T, effect: (val: T, prev: T | undefined) => void, options?: SignalOptions<T> & {
        render?: boolean;
    });
    write(value: T): T;
    _notify(state: number): void;
    _setError(error: unknown): void;
    _disposeNode(): void;
}
