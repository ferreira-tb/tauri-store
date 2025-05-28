import { computed } from 'vue';
import { globalStore, setGlobalStoreOptions } from './store';
import type { StoreBackendOptions, StoreRef, StoreRefOptions } from './types';

export function storeRef<T>(
  key: string,
  value: T,
  options?: StoreRefOptions,
  globalOptions?: StoreBackendOptions
): StoreRef<T> {
  if (options?.writeDefault ?? true) {
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
