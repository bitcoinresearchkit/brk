import type { Computed } from "./core.js";
export interface Heap {
    _heap: (Computed<unknown> | undefined)[];
    _marked: boolean;
    _min: number;
    _max: number;
}
export declare function increaseHeapSize(n: number, heap: Heap): void;
export declare function insertIntoHeap(n: Computed<any>, heap: Heap): void;
export declare function insertIntoHeapHeight(n: Computed<unknown>, heap: Heap): void;
export declare function deleteFromHeap(n: Computed<unknown>, heap: Heap): void;
export declare function markHeap(heap: Heap): void;
export declare function markNode(el: Computed<unknown>, newState?: number): void;
export declare function runHeap(heap: Heap, recompute: (el: Computed<unknown>) => void): void;
