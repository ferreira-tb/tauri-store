export { Store, store } from './store';

export { RuneStore, runeStore } from './runes';

export type {
  MaybePromise,
  Option,
  State,
  StoreBackendOptions,
  StoreContract,
  StoreFrontendOptions,
  StoreOptions,
  TauriPluginSvelteRuneStoreOptions,
  TauriPluginSvelteStoreOptions,
  TauriStoreContract,
} from './types';

export {
  clearAutosave,
  getDefaultSaveStrategy,
  getSaveStrategy,
  getStoreIds,
  getStorePath,
  getStoreState,
  getSveltePath,
  save,
  saveAll,
  saveAllNow,
  saveNow,
  setAutosave,
  setSaveStrategy,
  setStoreOptions,
  setSveltePath,
} from './commands';
