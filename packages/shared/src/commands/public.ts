import { invoke } from '@tauri-apps/api/core';
import type { State } from '@tauri-store/shared';

export function clearAutosave(plugin: string) {
  return function (): Promise<void> {
    return invoke(`plugin:${plugin}|clear_autosave`);
  };
}

export function getStoreIds(plugin: string) {
  return function (): Promise<string[]> {
    return invoke(`plugin:${plugin}|get_store_ids`);
  };
}

export function getStorePath(plugin: string) {
  return function (id: string): Promise<string> {
    return invoke(`plugin:${plugin}|get_store_path`, { id });
  };
}

export function getStoreState(plugin: string) {
  return function <T extends State>(id: string): Promise<T | null> {
    return invoke(`plugin:${plugin}|get_store_state`, { id });
  };
}

export function save(plugin: string) {
  return function (...id: (string | string[])[]): Promise<void> {
    const stores = id.flat(Number.POSITIVE_INFINITY);
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
