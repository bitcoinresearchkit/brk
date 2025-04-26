export declare function reconcile<T extends U, U>(value: T, key: string | ((item: NonNullable<any>) => any)): (state: U) => T;
