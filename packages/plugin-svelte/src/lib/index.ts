export { Store, store } from './store';

export { RuneStore, runeStore } from './runes';

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
  StoreContract,
  StoreFrontendOptions,
  StoreHooks,
  StoreKeyFilter,
  StoreKeyFilterStrategy,
  StoreOptions,
  TauriPluginSvelteRuneStoreOptions,
  TauriPluginSvelteStoreOptions,
  TauriStoreContract,
  TimeStrategy,
  TimeStrategyKind,
} from './types';
