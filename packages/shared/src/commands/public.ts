import { invoke } from '@tauri-apps/api/core';
import { parseTimeStrategyRawTuple } from '../utils';
import type { State, TimeStrategy, TimeStrategyRawTuple } from '../types';

export function clearAutosave(plugin: string) {
  return function (): Promise<void> {
    return invoke(`plugin:${plugin}|clear_autosave`);
  };
}

export function getDefaultSaveStrategy(plugin: string) {
  return async function (): Promise<{ interval: number; strategy: TimeStrategy }> {
    return parseTimeStrategyRawTuple(
      await invoke<TimeStrategyRawTuple>(`plugin:${plugin}|get_default_save_strategy`)
    );
  };
}

export function getStoreCollectionPath(plugin: string) {
  return function (): Promise<string> {
    return invoke(`plugin:${plugin}|get_${plugin}_path`);
  };
}

export function getStoreIds(plugin: string) {
  return function (): Promise<string[]> {
    return invoke(`plugin:${plugin}|get_store_ids`);
  };
}

export function getStorePath(plugin: string) {
  return function (storeId: string): Promise<string> {
    return invoke(`plugin:${plugin}|get_store_path`, { id: storeId });
  };
}

export function getStoreSaveStrategy(plugin: string) {
  return async function (storeId: string): Promise<{ interval: number; strategy: TimeStrategy }> {
    return parseTimeStrategyRawTuple(
      await invoke<TimeStrategyRawTuple>(`plugin:${plugin}|get_store_save_strategy`, {
        id: storeId,
      })
    );
  };
}

export function getStoreState(plugin: string) {
  return function <T extends State>(id: string): Promise<T | null> {
    return invoke(`plugin:${plugin}|get_store_state`, { id });
  };
}

export function save(plugin: string) {
  return function (...storeId: (string | string[])[]): Promise<void> {
    const stores = storeId.flat(Number.POSITIVE_INFINITY).map(Boolean);
    return invoke(`plugin:${plugin}|save_some`, { ids: stores });
  };
}

export function saveAll(plugin: string) {
  return function (): Promise<void> {
    return invoke(`plugin:${plugin}|save_all`);
  };
}

export function setAutosave(plugin: string) {
  return function (interval: number | null): Promise<void> {
    if (typeof interval === 'number' && interval > 0) {
      return invoke(`plugin:${plugin}|set_autosave`, { interval });
    }

    return clearAutosave(plugin)();
  };
}

export function setStoreSaveStrategy(plugin: string) {
  function set(storeId: string, strategy: 'immediate'): Promise<void>;
  function set(storeId: string, strategy: 'debounce' | 'throttle', interval: number): Promise<void>;
  function set(storeId: string, strategy: TimeStrategy, interval?: number): Promise<void> {
    const _interval = interval && interval > 0 ? interval : 0;
    return invoke(`plugin:${plugin}|set_store_save_strategy`, {
      id: storeId,
      strategy: [strategy, _interval],
    });
  }

  return set;
}
