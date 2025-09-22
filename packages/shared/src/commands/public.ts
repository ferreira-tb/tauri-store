import { flatten } from '../utils';
import { invoke } from '@tauri-apps/api/core';
import type { Option, State, StoreBackendOptions, StoreBackendRawOptions } from '../types';
import {
  isValidInterval,
  TimeStrategy,
  type TimeStrategyKind,
  type TimeStrategyRawTuple,
} from '../time-strategy';

export function allowSave(plugin: string) {
  return function (...storeId: string[]): Promise<void> {
    return invoke(`plugin:${plugin}|allow_save`, { ids: storeId });
  };
}

export function allowSync(plugin: string) {
  return function (...storeId: string[]): Promise<void> {
    return invoke(`plugin:${plugin}|allow_sync`, { ids: storeId });
  };
}

export function clearAutosave(plugin: string) {
  return function (): Promise<void> {
    return invoke(`plugin:${plugin}|clear_autosave`);
  };
}

export function denySave(plugin: string) {
  return function (...storeId: string[]): Promise<void> {
    return invoke(`plugin:${plugin}|deny_save`, { ids: storeId });
  };
}

export function denySync(plugin: string) {
  return function (...storeId: string[]): Promise<void> {
    return invoke(`plugin:${plugin}|deny_sync`, { ids: storeId });
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
    return invoke(`plugin:${plugin}|get_store_collection_path`);
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
  // TODO: use a saner signature.
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
  // TODO: use a saner signature.
  return function (...storeId: (string | string[])[]): Promise<void> {
    const args: { ids: string[] } = { ids: flatten(storeId) };
    return invoke(`plugin:${plugin}|save_some_now`, args);
  };
}

export function setAutosave(plugin: string) {
  return function (interval: Option<number>): Promise<void> {
    if (isValidInterval(interval)) {
      return invoke(`plugin:${plugin}|set_autosave`, { interval });
    }

    return clearAutosave(plugin)();
  };
}

// eslint-disable-next-line @typescript-eslint/explicit-module-boundary-types
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

export function setStoreCollectionPath(plugin: string) {
  return function (path: string): Promise<void> {
    return invoke(`plugin:${plugin}|set_store_collection_path`, { path });
  };
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
      saveOnExit: typeof options.saveOnExit === 'boolean' ? options.saveOnExit : null,
      saveStrategy,
    };

    return invoke(`plugin:${plugin}|set_store_options`, {
      id: storeId,
      options: _options,
    });
  };
}
