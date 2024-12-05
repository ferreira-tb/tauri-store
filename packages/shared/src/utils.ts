import type { StoreKeyFilter } from '@tauri-store/shared';
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

export function throttle<T extends (...args: any) => any>(f: T, wait?: number): T {
  return lodashThrottle(f, wait) as unknown as T;
}
