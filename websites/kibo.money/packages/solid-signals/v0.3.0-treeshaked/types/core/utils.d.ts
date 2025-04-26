export declare function isUndefined(value: any): value is undefined;
export type TryCatchResult<T, E> = [undefined, T] | [E];
export declare function tryCatch<T, E = Error>(fn: () => Promise<T>): Promise<TryCatchResult<T, E>>;
export declare function tryCatch<T, E = Error>(fn: () => T): TryCatchResult<T, E>;
