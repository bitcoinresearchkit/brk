import { Accessor, Setter } from "./v0.2.4-treeshaked/types/signals";

export type Signal<T> = Accessor<T> & { set: Setter<T>; reset: VoidFunction };
export type Signals = Awaited<typeof import("./wrapper.js").default>;
