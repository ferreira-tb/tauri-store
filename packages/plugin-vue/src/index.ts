export { storeRef } from './ref';
export { createStore, globalStore, setGlobalStoreOptions, store, type Store } from './store';

export {
  allowSave,
  allowSync,
  clearAutosave,
  denySave,
  denySync,
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
  StoreRef,
  StoreRefOptions,
  TauriPluginVueStoreOptions,
  TauriStoreContract,
  TimeStrategy,
  TimeStrategyKind,
} from './types';
