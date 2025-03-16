import type { TauriPluginPiniaStoreOptions } from './types';
import type { TauriStoreContract } from '@tauri-store/shared';

export { createPlugin, TauriPluginPinia } from './plugin';

export type {
  MaybePromise,
  Option,
  State,
  StoreBackendOptions,
  StoreFrontendOptions,
  StoreHooks,
  StoreOptions,
  TauriPluginPiniaOptions,
  TauriPluginPiniaStoreOptions,
  TauriStoreContract,
} from './types';

export {
  clearAutosave,
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
  setStoreCollectionPath,
  setStoreOptions,
} from './commands';

declare module 'pinia' {
  export interface PiniaCustomProperties {
    readonly $tauri: TauriStoreContract;
  }

  export interface DefineStoreOptionsBase<S, Store> {
    readonly tauri?: TauriPluginPiniaStoreOptions;
  }
}
