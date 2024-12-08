import { invoke } from '@tauri-apps/api/core';
import type { State } from '@tauri-store/shared';

/** @internal */
export function load(plugin: string) {
  return function <T extends State>(id: string): Promise<T> {
    return invoke(`plugin:${plugin}|load`, { id });
  };
}

/** @internal */
export function patch(plugin: string) {
  return function (id: string, state: State): Promise<void> {
    return invoke(`plugin:${plugin}|patch`, { id, state });
  };
}

/** @internal */
export function unload(plugin: string) {
  return function (id: string): Promise<void> {
    return invoke(`plugin:${plugin}|unload`, { id });
  };
}
