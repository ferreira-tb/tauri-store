import type { WatchOptions } from 'vue';
import type { PluginOptions, StoreOptions } from '@tauri-store/shared';

export type { StoreOptions } from '@tauri-store/shared';

export interface TauriPluginPiniaOptions extends PluginOptions {
  /** @default true */
  readonly deep?: WatchOptions['deep'];
  /** @default 'pre' */
  readonly flush?: WatchOptions['flush'];
}

export type TauriPluginPiniaStoreOptions = StoreOptions & TauriPluginPiniaOptions;
