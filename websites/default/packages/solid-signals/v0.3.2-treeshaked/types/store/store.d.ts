import { Computation } from "../core/index.js";
export type Store<T> = Readonly<T>;
export type StoreSetter<T> = (fn: (state: T) => void) => void;
type DataNode = Computation<any>;
type DataNodes = Record<PropertyKey, DataNode>;
declare const $RAW: unique symbol, $TRACK: unique symbol, $TARGET: unique symbol, $PROXY: unique symbol;
export declare const STORE_VALUE = "v", STORE_NODE = "n", STORE_HAS = "h";
export { $PROXY, $TRACK, $RAW, $TARGET };
export type StoreNode = {
    [STORE_VALUE]: Record<PropertyKey, any>;
    [STORE_NODE]?: DataNodes;
    [STORE_HAS]?: DataNodes;
};
export declare namespace SolidStore {
    interface Unwrappable {
    }
}
export type NotWrappable = string | number | bigint | symbol | boolean | Function | null | undefined | SolidStore.Unwrappable[keyof SolidStore.Unwrappable];
export declare function wrap<T extends Record<PropertyKey, any>>(value: T): T;
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
export declare function unwrap<T>(item: T, deep?: boolean, set?: Set<unknown>): T;
export declare function createStore<T extends object = {}>(store: T | Store<T>): [get: Store<T>, set: StoreSetter<T>];
export declare function createStore<T extends object = {}>(fn: (store: T) => void, store: T | Store<T>): [get: Store<T>, set: StoreSetter<T>];
export declare function deep<T extends object>(store: Store<T>): Store<any>;
