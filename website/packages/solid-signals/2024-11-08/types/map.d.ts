import type { Accessor } from "./signals.js";
export type Maybe<T> = T | void | null | undefined | false;
/**
 * Reactive map helper that caches each list item by reference to reduce unnecessary mapping on
 * updates.
 *
 * @see {@link https://github.com/solidjs/x-reactivity#maparray}
 */
export declare function mapArray<Item, MappedItem>(list: Accessor<Maybe<readonly Item[]>>, map: (value: Accessor<Item>, index: Accessor<number>) => MappedItem, options?: {
    keyed?: boolean | ((item: Item) => any);
    name?: string;
}): Accessor<MappedItem[]>;
