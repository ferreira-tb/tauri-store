export type * from './types';

export { Store, store } from './store';

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
} from './commands';
