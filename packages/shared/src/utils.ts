import type { Option } from './types';
import { debounce as kitDebounce, throttle as kitThrottle } from 'es-toolkit';

export function debounce<T extends (...args: any) => any>(f: T, wait?: Option<number>): T {
  return kitDebounce(f, wait ?? 0) as unknown as T;
}

export function throttle<T extends (...args: any) => any>(f: T, wait?: Option<number>): T {
  return kitThrottle(f, wait ?? 0) as unknown as T;
}

/**
 * Merge properties from `source` into `target` if they don't already exist.
 *
 * This is a shallow merge.
 */
export function merge<T extends object>(target: T = {} as T, source: T = {} as T): T {
  for (const [key, value] of Object.entries(source)) {
    if (typeof (target as Record<string, unknown>)[key] === 'undefined') {
      Reflect.set(target, key, value);
    }
  }

  return target;
}

/**
 * @internal
 */
export function flatten<T>(array: (T | T[])[]): T[] {
  return array.flat(Number.POSITIVE_INFINITY).filter(Boolean) as T[];
}

/**
 * Flushes all pending promises.
 *
 * @internal
 */
export function flushPromises(): Promise<void> {
  return new Promise((resolve) => void setTimeout(resolve, 0));
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
