import { NOT_PENDING } from "./constants.js";
import { type IQueue, type Transition } from "./scheduler.js";
export interface Disposable {
    (): void;
}
export interface Link {
    _dep: Signal<unknown> | Computed<unknown>;
    _sub: Computed<unknown>;
    _nextDep: Link | null;
    _prevSub: Link | null;
    _nextSub: Link | null;
}
export interface SignalOptions<T> {
    id?: string;
    name?: string;
    equals?: ((prev: T, next: T) => boolean) | false;
    pureWrite?: boolean;
    unobserved?: () => void;
}
export interface RawSignal<T> {
    id?: string;
    _subs: Link | null;
    _subsTail: Link | null;
    _value: T;
    _error?: unknown;
    _statusFlags: number;
    _name?: string;
    _equals: false | ((a: T, b: T) => boolean);
    _pureWrite?: boolean;
    _unobserved?: () => void;
    _time: number;
    _transition: Transition | null;
    _pendingValue: T | typeof NOT_PENDING;
    _pendingCheck?: Signal<boolean> & {
        _set: (v: boolean) => void;
    };
    _pendingSignal?: Signal<T> & {
        _set: (v: T) => void;
    };
    _optimistic?: boolean;
}
export interface FirewallSignal<T> extends RawSignal<T> {
    _firewall: Computed<any>;
    _nextChild: FirewallSignal<unknown> | null;
}
export type Signal<T> = RawSignal<T> | FirewallSignal<T>;
export interface Owner {
    id?: string;
    _disposal: Disposable | Disposable[] | null;
    _parent: Owner | null;
    _context: Record<symbol | string, unknown>;
    _childCount: number;
    _queue: IQueue;
    _firstChild: Owner | null;
    _nextSibling: Owner | null;
    _pendingDisposal: Disposable | Disposable[] | null;
    _pendingFirstChild: Owner | null;
}
export interface Computed<T> extends RawSignal<T>, Owner {
    _deps: Link | null;
    _depsTail: Link | null;
    _flags: number;
    _height: number;
    _nextHeap: Computed<any> | undefined;
    _prevHeap: Computed<any>;
    _fn: (prev?: T) => T;
    _child: FirewallSignal<any> | null;
    _notifyQueue?: (statusFlagsChanged: boolean, prevStatusFlags: number) => void;
}
export interface Root extends Owner {
    _root: true;
    _parentComputed: Computed<any> | null;
    dispose(self?: boolean): void;
}
export declare let context: Owner | null;
export declare function recompute(el: Computed<any>, create?: boolean): void;
export declare function dispose(node: Computed<unknown>): void;
export declare function getNextChildId(owner: Owner): string;
export declare function computed<T>(fn: (prev?: T) => T): Computed<T>;
export declare function computed<T>(fn: (prev: T) => T, initialValue?: T, options?: SignalOptions<T>): Computed<T>;
export declare function asyncComputed<T>(asyncFn: (prev?: T, refreshing?: boolean) => T | Promise<T> | AsyncIterable<T>): Computed<T> & {
    _refresh: () => void;
};
export declare function asyncComputed<T>(asyncFn: (prev: T, refreshing?: boolean) => T | Promise<T> | AsyncIterable<T>, initialValue: T, options?: SignalOptions<T>): Computed<T> & {
    _refresh: () => void;
};
export declare function signal<T>(v: T, options?: SignalOptions<T>): Signal<T>;
export declare function signal<T>(v: T, options?: SignalOptions<T>, firewall?: Computed<any>): FirewallSignal<T>;
export declare function isEqual<T>(a: T, b: T): boolean;
/**
 * Returns the current value stored inside the given compute function without triggering any
 * dependencies. Use `untrack` if you want to also disable owner tracking.
 */
export declare function untrack<T>(fn: () => T): T;
export declare function read<T>(el: Signal<T> | Computed<T>): T;
export declare function setSignal<T>(el: Signal<T> | Computed<T>, v: T | ((prev: T) => T)): T;
export declare function getObserver(): Owner | null;
export declare function getOwner(): Owner | null;
export declare function onCleanup(fn: Disposable): Disposable;
export declare function createOwner(options?: {
    id: string;
}): Root;
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
export declare function runWithOwner<T>(owner: Owner | null, fn: () => T): T;
export declare function staleValues<T>(fn: () => T, set?: boolean): T;
export declare function pending<T>(fn: () => T): T;
export declare function isPending(fn: () => any): boolean;
