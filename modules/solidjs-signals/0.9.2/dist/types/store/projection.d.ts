import { type Store, type StoreOptions } from "./store.js";
export declare function createProjectionInternal<T extends object = {}>(fn: (draft: T) => void | T | Promise<void | T> | AsyncIterable<void | T>, initialValue?: T, options?: StoreOptions): {
    store: Readonly<T>;
    node: any;
};
/**
 * Creates a mutable derived value
 *
 * @see {@link https://github.com/solidjs/x-reactivity#createprojection}
 */
export declare function createProjection<T extends Object = {}>(fn: (draft: T) => void | T | Promise<void | T> | AsyncIterable<void | T>, initialValue?: T, options?: StoreOptions): Store<T>;
