import { Accessor, Setter } from "./2024-11-02/types/signals";

export type Signal<T> = Accessor<T> & { set: Setter<T>; reset: VoidFunction };
export type Signals = Awaited<typeof import("./wrapper.js").default>;
