import { type Computed, type Signal } from "../core/index.js";
export type Store<T> = Readonly<T>;
export type StoreSetter<T> = (fn: (state: T) => T | void) => void;
export type StoreOptions = {
    key?: string | ((item: NonNullable<any>) => any);
    all?: boolean;
};
type DataNode = Signal<any>;
type DataNodes = Record<PropertyKey, DataNode>;
export declare const $TRACK: unique symbol, $DEEP: unique symbol, $TARGET: unique symbol, $PROXY: unique symbol, $DELETED: unique symbol;
export declare const STORE_VALUE = "v", STORE_OVERRIDE = "o", STORE_NODE = "n", STORE_HAS = "h", STORE_WRAP = "w", STORE_LOOKUP = "l", STORE_FIREWALL = "f";
export type StoreNode = {
    [$PROXY]: any;
    [STORE_VALUE]: Record<PropertyKey, any>;
    [STORE_OVERRIDE]?: Record<PropertyKey, any>;
    [STORE_NODE]?: DataNodes;
    [STORE_HAS]?: DataNodes;
    [STORE_WRAP]?: (value: any, target?: StoreNode) => any;
    [STORE_LOOKUP]?: WeakMap<any, any>;
    [STORE_FIREWALL]?: () => Computed<any>;
};
export declare namespace SolidStore {
    interface Unwrappable {
    }
}
export type NotWrappable = string | number | bigint | symbol | boolean | Function | null | undefined | SolidStore.Unwrappable[keyof SolidStore.Unwrappable];
export declare function createStoreProxy<T extends object>(value: T, traps?: ProxyHandler<StoreNode>, extend?: Record<PropertyKey, any>): any;
export declare const storeLookup: WeakMap<object, any>;
export declare function wrap<T extends Record<PropertyKey, any>>(value: T, target?: StoreNode): T;
export declare function isWrappable<T>(obj: T | NotWrappable): obj is T;
export declare function getKeys(source: Record<PropertyKey, any>, override: Record<PropertyKey, any> | undefined, enumerable?: boolean): PropertyKey[];
export declare function getPropertyDescriptor(source: Record<PropertyKey, any>, override: Record<PropertyKey, any> | undefined, property: PropertyKey): PropertyDescriptor | undefined;
export declare const storeTraps: ProxyHandler<StoreNode>;
export declare function storeSetter<T extends object>(store: Store<T>, fn: (draft: T) => T | void): void;
export declare function createStore<T extends object = {}>(store: T | Store<T>): [get: Store<T>, set: StoreSetter<T>];
export declare function createStore<T extends object = {}>(fn: (store: T) => void | T, store: T | Store<T>, options?: StoreOptions): [get: Store<T>, set: StoreSetter<T>];
export declare function deep<T extends object>(store: Store<T>): Store<T>;
export {};
