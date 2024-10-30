export type Store<T> = Readonly<T>;
export type StoreSetter<T> = (fn: (state: T) => void) => void;
declare const $TRACK: unique symbol, $PROXY: unique symbol;
export { $PROXY, $TRACK };
export type StoreNode = Record<PropertyKey, any>;
export declare namespace SolidStore {
    interface Unwrappable {
    }
}
export type NotWrappable = string | number | bigint | symbol | boolean | Function | null | undefined | SolidStore.Unwrappable[keyof SolidStore.Unwrappable];
export declare function isWrappable<T>(obj: T | NotWrappable): obj is T;
/**
 * Returns the underlying data in the store without a proxy.
 * @param item store proxy object
 * @example
 * ```js
 * const initial = {z...};
 * const [state, setState] = createStore(initial);
 * initial === state; // => false
 * initial === unwrap(state); // => true
 * ```
 */
export declare function unwrap<T>(item: T, set?: Set<unknown>): T;
export declare function createStore<T extends object = {}>(store: T | Store<T>): [get: Store<T>, set: StoreSetter<T>];
export declare function createStore<T extends object = {}>(fn: (store: T) => void, store: T | Store<T>): [get: Store<T>, set: StoreSetter<T>];
/**
 * Creates a mutable derived value
 *
 * @see {@link https://github.com/solidjs/x-reactivity#createprojection}
 */
export declare function createProjection<T extends Object>(fn: (draft: T) => void, initialValue: T): Readonly<T>;
