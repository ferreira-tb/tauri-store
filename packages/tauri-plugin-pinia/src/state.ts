import type { State, TauriPluginPiniaStoreOptions } from './types';

export function applyOptions(state: State, options?: TauriPluginPiniaStoreOptions): State {
  if (options?.ignoreKey) {
    const result: State = {};
    for (const [key, value] of Object.entries(state)) {
      if (
        (typeof options.ignoreKey === 'string' && key === options.ignoreKey) ||
        (Array.isArray(options.ignoreKey) && options.ignoreKey.includes(key))
      ) {
        continue;
      }

      result[key] = value;
    }

    return result;
  }

  return state;
}
