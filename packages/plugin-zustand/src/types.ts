import type { State, StoreOptions } from '@tauri-store/shared';

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
export type TauriPluginZustandStoreOptions<S extends State> = StoreOptions<S>;
