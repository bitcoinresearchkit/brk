/**
 * Nodes for constructing a graph of reactive values and reactive computations.
 *
 * - The graph is acyclic.
 * - The user inputs new values into the graph by calling .write() on one more computation nodes.
 * - The user retrieves computed results from the graph by calling .read() on one or more computation nodes.
 * - The library is responsible for running any necessary computations so that .read() is up to date
 *   with all prior .write() calls anywhere in the graph.
 * - We call the input nodes 'roots' and the output nodes 'leaves' of the graph here.
 * - Changes flow from roots to leaves. It would be effective but inefficient to immediately
 *   propagate all changes from a root through the graph to descendant leaves. Instead, we defer
 *   change most change propagation computation until a leaf is accessed. This allows us to
 *   coalesce computations and skip altogether recalculating unused sections of the graph.
 * - Each computation node tracks its sources and its observers (observers are other
 *   elements that have this node as a source). Source and observer links are updated automatically
 *   as observer computations re-evaluate and call get() on their sources.
 * - Each node stores a cache state (clean/check/dirty) to support the change propagation algorithm:
 *
 * In general, execution proceeds in three passes:
 *
 *  1. write() propagates changes down the graph to the leaves
 *     direct children are marked as dirty and their deeper descendants marked as check
 *     (no computations are evaluated)
 *  2. read() requests that parent nodes updateIfNecessary(), which proceeds recursively up the tree
 *     to decide whether the node is clean (parents unchanged) or dirty (parents changed)
 *  3. updateIfNecessary() evaluates the computation if the node is dirty (the computations are
 *     executed in root to leaf order)
 */
import { type Flags } from "./flags.js";
import { Owner } from "./owner.js";
import { type IQueue } from "./scheduler.js";
export interface SignalOptions<T> {
    name?: string;
    equals?: ((prev: T, next: T) => boolean) | false;
    unobserved?: () => void;
}
interface SourceType {
    _observers: ObserverType[] | null;
    _unobserved?: () => void;
    _updateIfNecessary: () => void;
    _stateFlags: Flags;
    _time: number;
}
interface ObserverType {
    _sources: SourceType[] | null;
    _notify: (state: number, skipQueue?: boolean) => void;
    _handlerMask: Flags;
    _notifyFlags: (mask: Flags, newFlags: Flags) => void;
    _time: number;
}
/**
 * Returns the current observer.
 */
export declare function getObserver(): Computation | null;
export declare const UNCHANGED: unique symbol;
export type UNCHANGED = typeof UNCHANGED;
export declare class Computation<T = any> extends Owner implements SourceType, ObserverType {
    _sources: SourceType[] | null;
    _observers: ObserverType[] | null;
    _value: T | undefined;
    _error: unknown;
    _compute: null | ((p?: T) => T);
    _name: string | undefined;
    _equals: false | ((a: T, b: T) => boolean);
    _unobserved: (() => void) | undefined;
    /** Whether the computation is an error or has ancestors that are unresolved */
    _stateFlags: number;
    /** Which flags raised by sources are handled, vs. being passed through. */
    _handlerMask: number;
    _loading: Computation<boolean> | null;
    _time: number;
    _forceNotify: boolean;
    constructor(initialValue: T | undefined, compute: null | ((p?: T) => T), options?: SignalOptions<T>);
    _read(): T;
    /**
     * Return the current value of this computation
     * Automatically re-executes the surrounding computation when the value changes
     */
    read(): T;
    /**
     * Return the current value of this computation
     * Automatically re-executes the surrounding computation when the value changes
     *
     * If the computation has any unresolved ancestors, this function waits for the value to resolve
     * before continuing
     */
    wait(): T;
    /**
     * Return true if the computation is the value is dependent on an unresolved promise
     * Triggers re-execution of the computation when the loading state changes
     *
     * This is useful especially when effects want to re-execute when a computation's
     * loading state changes
     */
    loading(): boolean;
    /** Update the computation with a new value. */
    write(value: T | ((currentValue: T) => T) | UNCHANGED, flags?: number, raw?: boolean): T;
    /**
     * Set the current node's state, and recursively mark all of this node's observers as STATE_CHECK
     */
    _notify(state: number, skipQueue?: boolean): void;
    /**
     * Notify the computation that one of its sources has changed flags.
     *
     * @param mask A bitmask for which flag(s) were changed.
     * @param newFlags The source's new flags, masked to just the changed ones.
     */
    _notifyFlags(mask: Flags, newFlags: Flags): void;
    _setError(error: unknown): void;
    /**
     * This is the core part of the reactivity system, which makes sure that the values are updated
     * before they are read. We've also adapted it to return the loading state of the computation,
     * so that we can propagate that to the computation's observers.
     *
     * This function will ensure that the value and states we read from the computation are up to date
     */
    _updateIfNecessary(): void;
    /**
     * Remove ourselves from the owner graph and the computation graph
     */
    _disposeNode(): void;
}
/**
 * Reruns a computation's _compute function, producing a new value and keeping track of dependencies.
 *
 * It handles the updating of sources and observers, disposal of previous executions,
 * and error handling if the _compute function throws. It also sets the node as loading
 * if it reads any parents that are currently loading.
 */
export declare function update<T>(node: Computation<T>): void;
export declare function isEqual<T>(a: T, b: T): boolean;
/**
 * Returns the current value stored inside the given compute function without triggering any
 * dependencies. Use `untrack` if you want to also disable owner tracking.
 */
export declare function untrack<T>(fn: () => T): T;
/**
 * Returns true if the given functinon contains signals that have been updated since the last time
 * the parent computation was run.
 */
export declare function hasUpdated(fn: () => any): boolean;
/**
 * Returns true if the given function contains async signals are out of date.
 */
export declare function isPending(fn: () => any): boolean;
export declare function isPending(fn: () => any, loadingValue: boolean): boolean;
/**
 * Attempts to resolve value of expression synchronously returning the last resolved value for any async computation.
 */
export declare function latest<T>(fn: () => T): T;
export declare function latest<T, U>(fn: () => T, fallback: U): T | U;
export declare function catchError(fn: () => void): unknown | undefined;
/**
 * Runs the given function in the given observer.
 *
 * Warning: Usually there are simpler ways of modeling a problem that avoid using this function
 */
export declare function runWithObserver<T>(observer: Computation, run: () => T): T | undefined;
/**
 * A convenient wrapper that calls `compute` with the `owner` and `observer` and is guaranteed
 * to reset the global context after the computation is finished even if an error is thrown.
 */
export declare function compute<T>(owner: Owner | null, fn: (val: T) => T, observer: Computation<T>): T;
export declare function compute<T>(owner: Owner | null, fn: (val: undefined) => T, observer: null): T;
export declare function flatten(children: any, options?: {
    skipNonRendered?: boolean;
    doNotUnwrap?: boolean;
}): any;
export declare function createBoundary<T>(fn: () => T, queue: IQueue): T;
export {};
