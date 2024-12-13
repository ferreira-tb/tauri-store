import type { TauriPluginPiniaStoreOptions } from './types';
import type { CustomStoreProperties } from '@tauri-store/shared';

export type * from './types';

export { createPlugin, TauriPluginPinia } from './plugin';

export {
  clearAutosave,
  getDefaultSaveStrategy,
  getPiniaPath,
  getSaveStrategy,
  getStoreIds,
  getStorePath,
  getStoreState,
  save,
  saveAll,
  saveAllNow,
  saveNow,
  setAutosave,
  setSaveStrategy,
} from './commands';

declare module 'pinia' {
  export interface PiniaCustomProperties {
    readonly $tauri: CustomStoreProperties;
  }

  export interface DefineStoreOptionsBase<S, Store> {
    readonly tauri?: TauriPluginPiniaStoreOptions;
  }
}
