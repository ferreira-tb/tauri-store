export { store, type Store } from './store';

export type {
  MaybePromise,
  Option,
  State,
  StoreBackendOptions,
  StoreBuilderReturn,
  StoreFrontendOptions,
  StoreHooks,
  StoreOptions,
  TauriPluginValtioStoreOptions,
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
