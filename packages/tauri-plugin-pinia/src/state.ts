import type { State, StoreKeyFilter, TauriPluginPiniaStoreOptions } from './types';

export function applyOptions(state: State, options?: TauriPluginPiniaStoreOptions): State {
  if (options?.filterKeys) {
    const result: State = {};
    const { filterKeysStrategy = 'omit' } = options;

    for (const [key, value] of Object.entries(state)) {
      if (
        (filterKeysStrategy === 'omit' && isStoreKeyMatch(options.filterKeys, key)) ||
        (filterKeysStrategy === 'pick' && !isStoreKeyMatch(options.filterKeys, key)) ||
        (typeof filterKeysStrategy === 'function' && !filterKeysStrategy(key))
      ) {
        continue;
      }

      result[key] = value;
    }

    return result;
  }

  return state;
}

function isStoreKeyMatch(filter: StoreKeyFilter, key: string): boolean {
  return (
    (typeof filter === 'string' && key === filter) ||
    (Array.isArray(filter) && filter.includes(key)) ||
    (filter instanceof RegExp && filter.test(key))
  );
}
