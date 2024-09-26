import type { State, TauriPluginPiniaStoreOptions } from './types';

export function applyOptions(state: State, options?: TauriPluginPiniaStoreOptions): State {
  if (options?.ignoreKeys) {
    const result: State = {};
    for (const [key, value] of Object.entries(state)) {
      if (
        (typeof options.ignoreKeys === 'string' && key === options.ignoreKeys) ||
        (Array.isArray(options.ignoreKeys) && options.ignoreKeys.includes(key))
      ) {
        continue;
      }

      result[key] = value;
    }

    return result;
  }

  return state;
}
