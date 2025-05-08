export { store, type Store, toStore } from './store';

export type {
  LooseTimeStrategyKind,
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
  TimeStrategyKind,
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
