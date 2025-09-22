import type { TauriPluginPiniaStoreOptions } from './types';
import type { TauriStoreContract } from '@tauri-store/shared';

export { createPlugin, TauriPluginPinia } from './plugin';

export {
  allowSave,
  allowSync,
  clearAutosave,
  denySave,
  denySync,
  destroy,
  getDefaultSaveStrategy,
  getSaveStrategy,
  getStoreCollectionPath,
  getStoreIds,
  getStorePath,
  getStoreState,
  save,
  saveAll,
  saveAllNow,
  saveNow,
  setAutosave,
  setSaveStrategy,
  setStoreOptions,
} from './commands';

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
  TauriPluginPiniaOptions,
  TauriPluginPiniaStoreOptions,
  TauriStoreContract,
  TimeStrategy,
  TimeStrategyKind,
} from './types';

declare module 'pinia' {
  export interface PiniaCustomProperties {
    readonly $tauri: TauriStoreContract;
  }

  export interface DefineStoreOptionsBase<S, Store> {
    readonly tauri?: TauriPluginPiniaStoreOptions;
  }
}
