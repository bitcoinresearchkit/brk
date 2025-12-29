import { type Computed, type Owner, type SignalOptions } from "./core.js";
export interface Effect<T> extends Computed<T>, Owner {
    _effectFn: (val: T, prev: T | undefined) => void | (() => void);
    _errorFn?: (err: unknown, cleanup: () => void) => void;
    _cleanup?: () => void;
    _modified: boolean;
    _prevValue: T | undefined;
    _type: number;
}
/**
 * Effects are the leaf nodes of our reactive graph. When their sources change, they are
 * automatically added to the queue of effects to re-execute, which will cause them to fetch their
 * sources and recompute
 */
export declare function effect<T>(compute: (prev: T | undefined) => T, effect: (val: T, prev: T | undefined) => void | (() => void), error?: (err: unknown, cleanup: () => void) => void | (() => void), initialValue?: T, options?: SignalOptions<any> & {
    render?: boolean;
    defer?: boolean;
}): void;
