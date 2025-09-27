import { type Store } from "./store.js";
/**
 * Creates a mutable derived value
 *
 * @see {@link https://github.com/solidjs/x-reactivity#createprojection}
 */
export declare function createProjection<T extends Object>(fn: (draft: T) => void, initialValue?: T): Store<T>;
