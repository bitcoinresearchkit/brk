import type { Accessor } from './signals';
export interface SelectorSignal<T> {
    (key: T): Boolean;
}
export interface SelectorOptions<Key, Value> {
    name?: string;
    equals?: (key: Key, value: Value | undefined) => boolean;
}
/**
 * Creates a signal that observes the given `source` and returns a new signal who only notifies
 * observers when entering or exiting a specified key.
 *
 * @see {@link https://github.com/solidjs/x-reactivity#createselector}
 */
export declare function createSelector<Source, Key = Source>(source: Accessor<Source>, options?: SelectorOptions<Key, Source>): SelectorSignal<Key>;
