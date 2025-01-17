export { Store, store } from './store';

export type {
  MaybePromise,
  Option,
  State,
  StoreBackendOptions,
  StoreContract,
  StoreFrontendOptions,
  StoreOptions,
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
