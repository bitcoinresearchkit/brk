import { Queue, type Computed, type Effect } from "./core/index.js";
import type { Signal } from "./core/index.js";
export interface BoundaryComputed<T> extends Computed<T> {
    _propagationMask: number;
}
export declare class CollectionQueue extends Queue {
    _collectionType: number;
    _nodes: Set<Effect<any>>;
    _disabled: Signal<boolean>;
    _initialized: boolean;
    constructor(type: number);
    run(type: number): void;
    notify(node: Effect<any>, type: number, flags: number): boolean;
}
export declare const enum BoundaryMode {
    VISIBLE = "visible",
    HIDDEN = "hidden"
}
export declare function createBoundary<T>(fn: () => T, condition: () => BoundaryMode): () => T | undefined;
export declare function createLoadBoundary(fn: () => any, fallback: () => any): () => unknown;
export declare function createErrorBoundary<U>(fn: () => any, fallback: (error: unknown, reset: () => void) => U): () => unknown;
export declare function flatten(children: any, options?: {
    skipNonRendered?: boolean;
    doNotUnwrap?: boolean;
}): any;
