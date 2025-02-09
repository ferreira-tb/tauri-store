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
  getStoreIds,
  getStorePath,
  getStoreState,
  getValtioPath,
  save,
  saveAll,
  saveAllNow,
  saveNow,
  setAutosave,
  setSaveStrategy,
  setStoreOptions,
  setValtioPath,
} from './commands';
