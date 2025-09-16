import { Computation, Queue } from "./core/index.js";
import type { Effect } from "./core/index.js";
export declare class CollectionQueue extends Queue {
    _collectionType: number;
    _nodes: Set<Effect>;
    _disabled: Computation<boolean>;
    constructor(type: number);
    run(type: number): void;
    notify(node: Effect, type: number, flags: number): boolean;
}
export declare enum BoundaryMode {
    VISIBLE = "visible",
    HIDDEN = "hidden"
}
export declare function createBoundary<T>(fn: () => T, condition: () => BoundaryMode): () => T | undefined;
export declare function createSuspense(fn: () => any, fallback: () => any): () => any;
export declare function createErrorBoundary<U>(fn: () => any, fallback: (error: unknown, reset: () => void) => U): () => any;
export declare function flatten(children: any, options?: {
    skipNonRendered?: boolean;
    doNotUnwrap?: boolean;
}): any;
