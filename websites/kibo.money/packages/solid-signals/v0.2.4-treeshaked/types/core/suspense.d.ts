import { Computation } from "./core.js";
import { type Effect } from "./effect.js";
import { Queue } from "./scheduler.js";
export declare class SuspenseQueue extends Queue {
    _nodes: Set<Effect>;
    _fallback: boolean;
    _signal: Computation<boolean>;
    run(type: number): true | undefined;
    _update(node: Effect): void;
}
export declare function createSuspense(fn: () => any, fallback: () => any): () => any;
