export { store, type Store, toStore } from './store';

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

export type {
  LooseTimeStrategyKind,
  MaybePromise,
  Nil,
  Option,
  State,
  StoreBackendOptions,
  StoreBuilderReturn,
  StoreFrontendOptions,
  StoreHooks,
  StoreKeyFilter,
  StoreKeyFilterStrategy,
  StoreOptions,
  TauriPluginValtioStoreOptions,
  TauriStoreContract,
  TimeStrategy,
  TimeStrategyKind,
} from './types';
