import { type Store, type StoreSetter } from "./store.js";
/**
 * Creates a mutable derived value
 *
 * @see {@link https://github.com/solidjs/x-reactivity#createprojection}
 */
export declare function createProjection<T extends Object>(fn: (draft: T) => void, initialValue?: T): Store<T>;
export declare function wrapProjection<T>(fn: (draft: T) => void, store: Store<T>, setStore: StoreSetter<T>): [Store<T>, StoreSetter<T>];
