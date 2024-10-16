export type Store<T> = Readonly<T>;
export type StoreSetter<T> = (fn: (state: T) => void) => void;
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
