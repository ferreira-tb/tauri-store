export * from './types';
import { debounce as lodashDebounce, throttle as lodashThrottle } from 'lodash-es';

// FIXME: We should not depend on lodash for this. A custom implementation would be better.
export function debounce<T extends (...args: any) => any>(f: T, wait?: number): T {
  return lodashDebounce(f, wait) as unknown as T;
}

export function throttle<T extends (...args: any) => any>(f: T, wait?: number): T {
  return lodashThrottle(f, wait) as unknown as T;
}
