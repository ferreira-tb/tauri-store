import type { WatchOptions } from 'vue';
import type { StoreOptions } from '@tauri-store/shared';

export type {
  LooseTimeStrategyKind,
  MaybePromise,
  nil,
  Option,
  State,
  StoreBackendOptions,
  StoreFrontendOptions,
  StoreHooks,
  StoreKeyFilter,
  StoreKeyFilterStrategy,
  StoreOptions,
  TauriStoreContract,
  TimeStrategy,
  TimeStrategyKind,
} from '@tauri-store/shared';

/**
 * Options for the Pinia plugin.
 */
export interface TauriPluginPiniaOptions extends StoreOptions {
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

/**
 * Options for the Pinia store.
 */
export type TauriPluginPiniaStoreOptions = TauriPluginPiniaOptions;
