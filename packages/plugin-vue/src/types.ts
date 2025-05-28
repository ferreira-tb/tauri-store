import type { Ref, WatchOptions } from 'vue';
import type { State, StoreOptions, TauriStoreContract } from '@tauri-store/shared';

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
 * Options for the store.
 */
export interface TauriPluginVueStoreOptions<S extends State> extends StoreOptions<S> {
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

export type StoreRef<T> = Ref<T> & { $tauri: TauriStoreContract };

export interface StoreRefOptions {
  /**
   * Write the default value to the store if its current value is nullish.
   * @default true
   */
  writeDefault?: boolean;
}
