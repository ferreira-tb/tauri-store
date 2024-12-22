import { debounce as lodashDebounce, throttle as lodashThrottle } from 'lodash-es';

// FIXME: We should not depend on lodash for this. A custom implementation would be better.
export function debounce<T extends (...args: any) => any>(f: T, wait?: number): T {
  return lodashDebounce(f, wait) as unknown as T;
}

// FIXME: Same as debounce. We need a custom implementation.
export function throttle<T extends (...args: any) => any>(f: T, wait?: number): T {
  return lodashThrottle(f, wait) as unknown as T;
}

/**
 * @internal
 */
export function parseBigInt(value: string): number {
  if (BigInt(value) > Number.MAX_SAFE_INTEGER) {
    throw new TypeError(`value ${value} is too large to be represented as a number`);
  }

  return Number.parseInt(value, 10);
}

/**
 * @internal
 */
export function flatten<T>(array: (T | T[])[]): T[] {
  return array.flat(Number.POSITIVE_INFINITY).filter(Boolean) as T[];
}
