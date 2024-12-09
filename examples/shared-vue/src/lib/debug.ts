import {
  clearAutosave,
  getDefaultSaveStrategy,
  getPiniaPath,
  getStoreIds,
  getStorePath,
  getStoreSaveStrategy,
  getStoreState,
  save,
  saveAll,
  setAutosave,
  setStoreSaveStrategy,
} from 'tauri-plugin-pinia';

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
        getStoreIds,
        getStorePath,
        getStoreSaveStrategy,
        getStoreState,
        save,
        saveAll,
        setAutosave,
        setStoreSaveStrategy,
      },
    });
  }
}
