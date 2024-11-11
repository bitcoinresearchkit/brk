/**
 * Owner tracking is used to enable nested tracking scopes with automatic cleanup.
 * We also use owners to also keep track of which error handling context we are in.
 *
 * If you write the following
 *
 *   const a = createOwner(() => {
 *     const b = createOwner(() => {});
 *
 *     const c = createOwner(() => {
 *       const d = createOwner(() => {});
 *     });
 *
 *     const e = createOwner(() => {});
 *   });
 *
 * The owner tree will look like this:
 *
 *    a
 *   /|\
 *  b-c-e
 *    |
 *    d
 *
 * Following the _nextSibling pointers of each owner will first give you its children, and then its siblings (in reverse).
 * a -> e -> c -> d -> b
 *
 * Note that the owner tree is largely orthogonal to the reactivity tree, and is much closer to the component tree.
 */
import { type ErrorHandler } from "./error.js";
export type ContextRecord = Record<string | symbol, unknown>;
export interface Disposable {
    (): void;
}
/**
 * Returns the currently executing parent owner.
 */
export declare function getOwner(): Owner | null;
export declare function setOwner(owner: Owner | null): Owner | null;
export declare class Owner {
    _parent: Owner | null;
    _nextSibling: Owner | null;
    _prevSibling: Owner | null;
    _state: number;
    _disposal: Disposable | Disposable[] | null;
    _context: ContextRecord;
    _handlers: ErrorHandler[] | null;
    constructor(signal?: boolean);
    append(child: Owner): void;
    dispose(this: Owner, self?: boolean): void;
    _disposeNode(): void;
    emptyDisposal(): void;
    handleError(error: unknown): void;
}
export interface Context<T> {
    readonly id: symbol;
    readonly defaultValue: T | undefined;
}
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
/**
 * Whether the given context is currently defined.
 */
export declare function hasContext(context: Context<any>, owner?: Owner | null): boolean;
/**
 * Runs the given function when the parent owner computation is being disposed.
 */
export declare function onCleanup(disposable: Disposable): void;
