import type { State, StoreOptions, TauriStoreContract } from '@tauri-store/shared';

export type {
  MaybePromise,
  Option,
  State,
  StoreBackendOptions,
  StoreFrontendOptions,
  StoreHooks,
  StoreOptions,
  TauriStoreContract,
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
}
