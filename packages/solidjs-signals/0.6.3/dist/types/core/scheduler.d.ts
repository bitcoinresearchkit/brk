import type { Computation, ObserverType, SourceType } from "./core.js";
import type { Effect } from "./effect.js";
export declare let clock: number;
export declare function incrementClock(): void;
export declare let ActiveTransition: Transition | null;
export declare let Unobserved: SourceType[];
export type QueueCallback = (type: number) => void;
export interface IQueue {
    enqueue(type: number, fn: QueueCallback): void;
    run(type: number): boolean | void;
    flush(): void;
    addChild(child: IQueue): void;
    removeChild(child: IQueue): void;
    created: number;
    notify(...args: any[]): boolean;
    merge(queue: IQueue): void;
    _parent: IQueue | null;
    _cloned?: IQueue | undefined;
}
export declare class Queue implements IQueue {
    _parent: IQueue | null;
    _running: boolean;
    _queues: [QueueCallback[], QueueCallback[]];
    _children: IQueue[];
    created: number;
    enqueue(type: number, fn: QueueCallback): void;
    run(type: number): void;
    flush(): void;
    addChild(child: IQueue): any;
    removeChild(child: IQueue): any;
    notify(...args: any[]): boolean;
    merge(queue: Queue): void;
}
export declare const globalQueue: Queue;
/**
 * By default, changes are batched on the microtask queue which is an async process. You can flush
 * the queue synchronously to get the latest updates by calling `flush()`.
 */
export declare function flush(): void;
export declare function removeSourceObservers(node: ObserverType, index: number): void;
export declare class Transition implements IQueue {
    _sources: Map<Computation, Computation>;
    _pendingNodes: Set<Effect>;
    _promises: Set<Promise<any>>;
    _optimistic: Set<(() => void) & {
        _transition?: Transition;
    }>;
    _done: Transition | boolean;
    _queues: [QueueCallback[], QueueCallback[]];
    _clonedQueues: Map<Queue, Queue>;
    _pureQueue: QueueCallback[];
    _children: IQueue[];
    _parent: IQueue | null;
    _running: boolean;
    _scheduled: boolean;
    _cloned: Queue;
    created: number;
    constructor();
    enqueue(type: number, fn: QueueCallback): void;
    run(type: number): void;
    flush(): void;
    addChild(child: IQueue): void;
    removeChild(child: IQueue): void;
    notify(node: Effect, type: number, flags: number): boolean;
    merge(queue: Transition): void;
    schedule(): void;
    runTransition(fn: () => any | Promise<any>, force?: boolean): void;
    addOptimistic(fn: (() => void) & {
        _transition?: Transition;
    }): void;
}
/**
 * Runs the given function in a transition scope, allowing for batch updates and optimizations.
 * This is useful for grouping multiple state updates together to avoid unnecessary re-renders.
 *
 * @param fn A function that receives a resume function to continue the transition.
 * The resume function can be called with another function to continue the transition.
 *
 * @description https://docs.solidjs.com/reference/advanced-reactivity/transition
 */
export declare function transition(fn: (resume: (fn: () => any | Promise<any>) => void) => any | Promise<any> | Iterable<any>): void;
export declare function cloneGraph(node: Computation): Computation;
export declare function getOGSource<T extends Computation>(input: T): T;
export declare function getTransitionSource<T extends Computation>(input: T): T;
export declare function getQueue(node: Computation): IQueue;
export declare function initialDispose(node: any): void;
