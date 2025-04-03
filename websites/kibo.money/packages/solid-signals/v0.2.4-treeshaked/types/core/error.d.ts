import type { Owner } from "./owner.js";
export declare class NotReadyError extends Error {
}
export declare class NoOwnerError extends Error {
    constructor();
}
export declare class ContextNotFoundError extends Error {
    constructor();
}
export declare class EffectError extends Error {
    constructor(effect: Function, cause: unknown);
}
export interface ErrorHandler {
    (error: unknown, node: Owner): void;
}
