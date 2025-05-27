import { computed } from 'vue';
import { globalStore, setGlobalStoreOptions } from './store';
import type { StoreBackendOptions, StoreRef } from './types';

export interface StoreRefOptions {
  /**
   * Write the default value to the store when it does not exist.
   * @default true
   */
  writeDefaults?: boolean;
}

export function storeRef<T>(
  key: string,
  value: T,
  options?: StoreRefOptions,
  globalOptions?: StoreBackendOptions
): StoreRef<T> {
  if (options?.writeDefaults ?? true) {
    globalStore.value[key] ??= value;
  }

  const computedRef = computed<T>({
    get: () => globalStore.value[key] as T,
    set: (newValue: T) => {
      globalStore.value[key] = newValue;
    },
  });

  if (globalOptions) {
    void setGlobalStoreOptions(globalOptions);
  }

  return Object.assign(computedRef, { $tauri: globalStore.$tauri });
}
