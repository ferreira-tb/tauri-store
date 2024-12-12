import { flatten } from '../utils';
import type { Option } from '@tb-dev/utils';
import { invoke } from '@tauri-apps/api/core';
import type { State, StoreBackendOptions, StoreBackendRawOptions } from '../types';
import {
  TimeStrategy,
  type TimeStrategyKind,
  type TimeStrategyRawTuple,
} from '../utils/time-strategy';

export function clearAutosave(plugin: string) {
  return function (): Promise<void> {
    return invoke(`plugin:${plugin}|clear_autosave`);
  };
}

export function getDefaultSaveStrategy(plugin: string) {
  return async function (): Promise<TimeStrategy> {
    return TimeStrategy.parse(
      await invoke<TimeStrategyRawTuple>(`plugin:${plugin}|get_default_save_strategy`)
    );
  };
}

export function getSaveStrategy(plugin: string) {
  return async function (storeId: string): Promise<TimeStrategy> {
    return TimeStrategy.parse(
      await invoke<TimeStrategyRawTuple>(`plugin:${plugin}|get_save_strategy`, { id: storeId })
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

export function getStoreState(plugin: string) {
  return function <T extends State>(storeId: string): Promise<T> {
    return invoke(`plugin:${plugin}|get_store_state`, { id: storeId });
  };
}

export function save(plugin: string) {
  return function (...storeId: (string | string[])[]): Promise<void> {
    const args: { ids: string[] } = { ids: flatten(storeId) };
    return invoke(`plugin:${plugin}|save_some`, args);
  };
}

export function saveAll(plugin: string) {
  return function (): Promise<void> {
    return invoke(`plugin:${plugin}|save_all`);
  };
}

export function saveAllNow(plugin: string) {
  return function (): Promise<void> {
    return invoke(`plugin:${plugin}|save_all_now`);
  };
}

export function saveNow(plugin: string) {
  return function (...storeId: (string | string[])[]): Promise<void> {
    const args: { ids: string[] } = { ids: flatten(storeId) };
    return invoke(`plugin:${plugin}|save_some_now`, args);
  };
}

export function setAutosave(plugin: string) {
  return function (interval: Option<number>): Promise<void> {
    if (typeof interval === 'number' && interval > 0) {
      return invoke(`plugin:${plugin}|set_autosave`, { interval });
    }

    return clearAutosave(plugin)();
  };
}

export function setSaveStrategy(plugin: string) {
  function set(storeId: string, strategy: 'immediate'): Promise<void>;
  function set(storeId: string, strategy: 'debounce' | 'throttle', interval: number): Promise<void>;
  function set(storeId: string, strategy: TimeStrategyKind, interval?: number): Promise<void> {
    const timeStrategy = new TimeStrategy(strategy, interval);
    return invoke(`plugin:${plugin}|set_save_strategy`, {
      id: storeId,
      strategy: timeStrategy.tuple(),
    });
  }

  return set;
}

export function setStoreOptions(plugin: string) {
  return function (storeId: string, options: StoreBackendOptions): Promise<void> {
    let saveStrategy: TimeStrategyRawTuple | undefined;
    if (options.saveStrategy ?? typeof options.saveInterval === 'number') {
      const timeStrategy = new TimeStrategy(options.saveStrategy, options.saveInterval);
      saveStrategy = timeStrategy.tuple();
    }

    const _options: StoreBackendRawOptions = {
      saveOnChange: typeof options.saveOnChange === 'boolean' ? options.saveOnChange : null,
      saveStrategy,
    };

    return invoke(`plugin:${plugin}|set_store_options`, {
      id: storeId,
      options: _options,
    });
  };
}
