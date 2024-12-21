import type { WatchOptions } from 'vue';
import type { StoreFrontendOptions, StoreOptions } from '@tauri-store/shared';

export type { StoreOptions } from '@tauri-store/shared';

export interface TauriPluginPiniaOptions extends StoreFrontendOptions {
  /** @default true */
  readonly deep?: WatchOptions['deep'];
  /** @default 'pre' */
  readonly flush?: WatchOptions['flush'];
}

export type TauriPluginPiniaStoreOptions = StoreOptions & TauriPluginPiniaOptions;
