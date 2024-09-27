import type { Writeable } from '@tb-dev/utils';
import type { State, StoreKeyFilter, TauriPluginPiniaStoreOptions } from './types';

export function applyOptions(state: State, options?: TauriPluginPiniaStoreOptions): State {
  if (options?.ignoreKeys) {
    console.warn('The `ignoreKeys` option is deprecated. Use `filterKeys` instead.');
    (options as Writeable<typeof options>).filterKeys ??= options.ignoreKeys;
    (options as Writeable<typeof options>).filterKeysStrategy ??= 'omit';
  }

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
