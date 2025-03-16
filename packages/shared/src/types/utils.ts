/** Void function. */
export type Fn = () => void;

/** Something may be a promise. */
export type MaybePromise<T> = T | PromiseLike<T>;

/** Null or undefined. */
export type Nil = null | undefined;

/** Something may be nullish. */
export type Option<T> = T | Nil;

/** Removes the readonly modifier from all properties of T. */
export type Writable<T> = { -readonly [P in keyof T]: T[P] };
