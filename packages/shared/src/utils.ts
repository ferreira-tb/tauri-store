import type { StoreKeyFilter, TimeStrategy, TimeStrategyRawTuple } from './types';
import { debounce as lodashDebounce, throttle as lodashThrottle } from 'lodash-es';

export function isStoreKeyMatch(filter: StoreKeyFilter, key: string): boolean {
  return (
    (typeof filter === 'string' && key === filter) ||
    (Array.isArray(filter) && filter.includes(key)) ||
    (filter instanceof RegExp && filter.test(key))
  );
}

// FIXME: We should not depend on lodash for this. A custom implementation would be better.
export function debounce<T extends (...args: any) => any>(f: T, wait?: number): T {
  return lodashDebounce(f, wait) as unknown as T;
}

// FIXME: Same as debounce. We need a custom implementation.
export function throttle<T extends (...args: any) => any>(f: T, wait?: number): T {
  return lodashThrottle(f, wait) as unknown as T;
}

/**
 * Rust represents milliseconds as an unsigned 128-bit integer,
 * which is too large to be represented as a number in JavaScript.
 *
 * For this reason, it must return the value as a string.
 *
 * @internal
 */
export function parseTimeStrategyRawTuple(tuple: TimeStrategyRawTuple): {
  interval: number;
  strategy: TimeStrategy;
} {
  return {
    interval: parseBigInt(tuple[1]),
    strategy: tuple[0],
  };
}

/**
 * @internal
 */
export function parseBigInt(value: string): number {
  if (BigInt(value) > Number.MAX_SAFE_INTEGER) {
    throw new TypeError(`Value ${value} is too large to be represented as a number`);
  }

  return Number.parseInt(value, 10);
}

/**
 * @internal
 */
export function flatten<T>(array: (T | T[])[]): T[] {
  return array.flat(Number.POSITIVE_INFINITY).filter(Boolean) as T[];
}
