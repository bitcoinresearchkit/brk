import { type Owner } from "./core.js";
export interface Context<T> {
    readonly id: symbol;
    readonly defaultValue: T | undefined;
}
export type ContextRecord = Record<string | symbol, unknown>;
/**
 * Context provides a form of dependency injection. It is used to save from needing to pass
 * data as props through intermediate components. This function creates a new context object
 * that can be used with `getContext` and `setContext`.
 *
 * A default value can be provided here which will be used when a specific value is not provided
 * via a `setContext` call.
 */
export declare function createContext<T>(defaultValue?: T, description?: string): Context<T>;
/**
 * Attempts to get a context value for the given key.
 *
 * @throws `NoOwnerError` if there's no owner at the time of call.
 * @throws `ContextNotFoundError` if a context value has not been set yet.
 */
export declare function getContext<T>(context: Context<T>, owner?: Owner | null): T;
/**
 * Attempts to set a context value on the parent scope with the given key.
 *
 * @throws `NoOwnerError` if there's no owner at the time of call.
 */
export declare function setContext<T>(context: Context<T>, value?: T, owner?: Owner | null): void;
