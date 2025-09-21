export declare function getClock(): number;
export declare function incrementClock(): void;
type QueueCallback = (type: number) => void;
export interface IQueue {
    enqueue(type: number, fn: QueueCallback): void;
    run(type: number): boolean | void;
    flush(): void;
    addChild(child: IQueue): void;
    removeChild(child: IQueue): void;
    created: number;
    notify(...args: any[]): boolean;
    _parent: IQueue | null;
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
    addChild(child: IQueue): void;
    removeChild(child: IQueue): void;
    notify(...args: any[]): boolean;
}
export declare const globalQueue: Queue;
/**
 * By default, changes are batched on the microtask queue which is an async process. You can flush
 * the queue synchronously to get the latest updates by calling `flush()`.
 */
export declare function flush(): void;
export {};
