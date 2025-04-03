import type { Computation } from "./core.js";
import type { Effect } from "./effect.js";
export declare function getClock(): number;
export declare function incrementClock(): void;
export interface IQueue {
    enqueue<T extends Computation | Effect>(type: number, node: T): void;
    run(type: number): boolean | void;
    flush(): void;
    addChild(child: IQueue): void;
    removeChild(child: IQueue): void;
    created: number;
}
export declare class Queue implements IQueue {
    _running: boolean;
    _queues: [Computation[], Effect[], Effect[]];
    _children: IQueue[];
    created: number;
    enqueue<T extends Computation | Effect>(type: number, node: T): void;
    run(type: number): true | undefined;
    flush(): void;
    addChild(child: IQueue): void;
    removeChild(child: IQueue): void;
}
export declare const globalQueue: Queue;
/**
 * By default, changes are batched on the microtask queue which is an async process. You can flush
 * the queue synchronously to get the latest updates by calling `flushSync()`.
 */
export declare function flushSync(): void;
