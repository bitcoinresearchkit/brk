export declare function reconcile<T extends U, U>(value: T, key: string | ((item: NonNullable<any>) => any), all?: boolean): (state: U) => void;
