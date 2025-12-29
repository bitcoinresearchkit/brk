import type { Computed, Signal } from "./core.js";
import { type Heap } from "./heap.js";
export declare const dirtyQueue: Heap;
export declare const zombieQueue: Heap;
export declare let clock: number;
export declare let activeTransition: Transition | null;
export type QueueCallback = (type: number) => void;
type QueueStub = {
    _queues: [QueueCallback[], QueueCallback[]];
    _children: QueueStub[];
};
export interface Transition {
    time: number;
    asyncNodes: Computed<any>[];
    pendingNodes: Signal<any>[];
    queueStash: QueueStub;
}
export declare function schedule(): void;
export interface IQueue {
    enqueue(type: number, fn: QueueCallback): void;
    run(type: number): boolean | void;
    addChild(child: IQueue): void;
    removeChild(child: IQueue): void;
    created: number;
    notify(node: Computed<any>, mask: number, flags: number): boolean;
    stashQueues(stub: QueueStub): void;
    restoreQueues(stub: QueueStub): void;
    _parent: IQueue | null;
}
export declare class Queue implements IQueue {
    _parent: IQueue | null;
    _queues: [QueueCallback[], QueueCallback[]];
    _children: IQueue[];
    created: number;
    addChild(child: IQueue): void;
    removeChild(child: IQueue): void;
    notify(node: Computed<any>, mask: number, flags: number): boolean;
    run(type: number): void;
    enqueue(type: number, fn: QueueCallback): void;
    stashQueues(stub: QueueStub): void;
    restoreQueues(stub: QueueStub): void;
}
export declare class GlobalQueue extends Queue {
    _running: boolean;
    _pendingNodes: Signal<any>[];
    static _update: (el: Computed<unknown>) => void;
    static _dispose: (el: Computed<unknown>, self: boolean, zombie: boolean) => void;
    flush(): void;
    notify(node: Computed<any>, mask: number, flags: number): boolean;
    initTransition(node: Computed<any>): void;
}
export declare const globalQueue: GlobalQueue;
/**
 * By default, changes are batched on the microtask queue which is an async process. You can flush
 * the queue synchronously to get the latest updates by calling `flush()`.
 */
export declare function flush(): void;
export declare function runInTransition(el: Computed<unknown>, recompute: (el: Computed<unknown>) => void): void;
export {};
