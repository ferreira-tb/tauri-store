export { Store, store } from './store';

export type {
  CloneFn,
  MaybePromise,
  Option,
  State,
  StoreBackendOptions,
  StoreFrontendOptions,
  StoreHooks,
  StoreOptions,
  TauriStoreContract,
  TauriStoreOptions,
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
