import type { WatchOptions } from 'vue';
import type { StoreFrontendOptions, StoreOptions } from '@tauri-store/shared';

export type {
  StoreBackendOptions,
  StoreFrontendOptions,
  StoreOptions,
  TauriStoreContract,
} from '@tauri-store/shared';

export interface TauriPluginPiniaOptions extends StoreFrontendOptions {
  /**
   * @see https://vuejs.org/api/reactivity-core.html#watch
   * @default true
   */
  readonly deep?: WatchOptions['deep'];

  /**
   * @see https://vuejs.org/api/reactivity-core.html#watch
   * @default 'pre'
   */
  readonly flush?: WatchOptions['flush'];
}

export type TauriPluginPiniaStoreOptions = StoreOptions & TauriPluginPiniaOptions;
