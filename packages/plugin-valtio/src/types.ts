import type { Store } from './store';
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
 * Options for the Valtio store.
 */
export type TauriPluginValtioStoreOptions<S extends State> = StoreOptions<S>;

export interface StoreBuilderReturn<S extends State> extends TauriStoreContract {
  /**
   * Valtio proxy state.
   *
   * @see https://valtio.dev/docs/api/basic/proxy
   */
  state: S;

  /**
   * Subscribes to changes in the store.
   */
  subscribe: Store<S>['subscribe'];

  /**
   * Subscribes to changes in a key of the store.
   */
  subscribeKey: Store<S>['subscribeKey'];
}
