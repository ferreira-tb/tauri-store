import {
  clearAutosave,
  getDefaultSaveStrategy,
  getPiniaPath,
  getSaveStrategy,
  getStoreIds,
  getStorePath,
  getStoreState,
  save,
  saveAll,
  saveAllNow,
  saveNow,
  setAutosave,
  setSaveStrategy,
} from 'tauri-plugin-pinia/src/index.js';

export function defineGlobalProperty() {
  if (!Object.hasOwn(window, '$p')) {
    Object.defineProperty(window, '$p', {
      configurable: false,
      enumerable: true,
      writable: false,
      value: {
        clearAutosave,
        getDefaultSaveStrategy,
        getPiniaPath,
        getSaveStrategy,
        getStoreIds,
        getStorePath,
        getStoreState,
        save,
        saveAll,
        saveAllNow,
        saveNow,
        setAutosave,
        setSaveStrategy,
      },
    });
  }
}
