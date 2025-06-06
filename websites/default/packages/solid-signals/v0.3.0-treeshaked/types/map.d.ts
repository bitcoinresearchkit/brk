import type { Accessor } from "./signals.js";
export type Maybe<T> = T | void | null | undefined | false;
/**
 * Reactively transforms an array with a callback function - underlying helper for the `<For>` control flow
 *
 * similar to `Array.prototype.map`, but gets the value and index as accessors, transforms only values that changed and returns an accessor and reactively tracks changes to the list.
 *
 * @description https://docs.solidjs.com/reference/reactive-utilities/map-array
 */
export declare function mapArray<Item, MappedItem>(list: Accessor<Maybe<readonly Item[]>>, map: (value: Accessor<Item>, index: Accessor<number>) => MappedItem, options?: {
    keyed?: boolean | ((item: Item) => any);
    fallback?: Accessor<any>;
}): Accessor<MappedItem[]>;
/**
 * Reactively repeats a callback function the count provided - underlying helper for the `<Repeat>` control flow
 *
 * @description https://docs.solidjs.com/reference/reactive-utilities/repeat
 */
export declare function repeat(count: Accessor<number>, map: (index: number) => any, options?: {
    from?: Accessor<number | undefined>;
    fallback?: Accessor<any>;
}): Accessor<any[]>;
