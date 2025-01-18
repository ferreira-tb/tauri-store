import type { State, StoreOptions, TauriStoreContract } from '@tauri-store/shared';

export type {
  MaybePromise,
  Option,
  State,
  StoreBackendOptions,
  StoreFrontendOptions,
  StoreOptions,
  TauriStoreContract,
} from '@tauri-store/shared';

/**
 * Options for the Valtio store.
 */
export type TauriPluginValtioStoreOptions = StoreOptions;

export interface StoreBuilderReturn<S extends State> extends TauriStoreContract {
  state: S;
}
