import { EFFECT_RENDER, EFFECT_USER } from "./constants.js";
import { Computation, type SignalOptions } from "./core.js";
/**
 * Effects are the leaf nodes of our reactive graph. When their sources change, they are
 * automatically added to the queue of effects to re-execute, which will cause them to fetch their
 * sources and recompute
 */
export declare class Effect<T = any> extends Computation<T> {
    _effect: (val: T, prev: T | undefined) => void | (() => void);
    _onerror: ((err: unknown, cleanup: () => void) => void) | undefined;
    _cleanup: (() => void) | undefined;
    _modified: boolean;
    _prevValue: T | undefined;
    _type: typeof EFFECT_RENDER | typeof EFFECT_USER;
    constructor(initialValue: T, compute: (val?: T) => T, effect: (val: T, prev: T | undefined) => void | (() => void), error?: (err: unknown) => void | (() => void), options?: SignalOptions<T> & {
        render?: boolean;
        defer?: boolean;
    });
    write(value: T, flags?: number): T;
    _notify(state: number, skipQueue?: boolean): void;
    _setError(error: unknown): void;
    _disposeNode(): void;
    _run(type: number): void;
}
export declare class EagerComputation<T = any> extends Computation<T> {
    constructor(initialValue: T, compute: () => T, options?: SignalOptions<T> & {
        defer?: boolean;
    });
    _notify(state: number, skipQueue?: boolean): void;
    _run(): void;
}
export declare class FirewallComputation extends Computation {
    firewall: boolean;
    constructor(compute: () => void);
    _notify(state: number, skipQueue?: boolean): void;
    _run(): void;
}
