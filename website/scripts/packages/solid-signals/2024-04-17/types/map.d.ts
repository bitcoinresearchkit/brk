import type { Accessor } from './signals';
export type Maybe<T> = T | void | null | undefined | false;
/**
 * Reactive map helper that caches each item by index to reduce unnecessary mapping on updates.
 * It only runs the mapping function once per item and adds/removes as needed. In a non-keyed map
 * like this the index is fixed but value can change (opposite of a keyed map).
 *
 * Prefer `mapArray` when referential checks are required.
 *
 * @see {@link https://github.com/solidjs/x-reactivity#indexarray}
 */
export declare function indexArray<Item, MappedItem>(list: Accessor<Maybe<readonly Item[]>>, map: (value: Accessor<Item>, index: number) => MappedItem, options?: {
    name?: string;
}): Accessor<MappedItem[]>;
/**
 * Reactive map helper that caches each list item by reference to reduce unnecessary mapping on
 * updates. It only runs the mapping function once per item and then moves or removes it as needed.
 * In a keyed map like this the value is fixed but the index changes (opposite of non-keyed map).
 *
 * Prefer `indexArray` when working with primitives to avoid unnecessary re-renders.
 *
 * @see {@link https://github.com/solidjs/x-reactivity#maparray}
 */
export declare function mapArray<Item, MappedItem>(list: Accessor<Maybe<readonly Item[]>>, map: (value: Item, index: Accessor<number>) => MappedItem, options?: {
    name?: string;
}): Accessor<MappedItem[]>;
