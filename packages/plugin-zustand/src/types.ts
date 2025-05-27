import type { State, StoreOptions } from '@tauri-store/shared';

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
export type TauriPluginZustandStoreOptions<S extends State> = StoreOptions<S>;
