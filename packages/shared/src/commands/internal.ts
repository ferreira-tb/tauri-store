import { invoke } from '@tauri-apps/api/core';
import type { State, TauriStoreContract } from '../types';

/**
 * Destroys the store.
 *
 * {@link TauriStoreContract.stop} **MUST** be called immediately after.
 *
 * @internal
 */
export function destroy(plugin: string) {
  return function (storeId: string): Promise<void> {
    return invoke(`plugin:${plugin}|destroy`, { id: storeId });
  };
}

/**
 * Loads the store state from the backend.
 *
 * @internal
 */
export function load(plugin: string) {
  return function <T extends State>(id: string): Promise<T> {
    return invoke(`plugin:${plugin}|load`, { id });
  };
}

/**
 * Patches the store state in the backend.
 *
 * @internal
 */
export function patch(plugin: string) {
  return function (id: string, state: State): Promise<void> {
    return invoke(`plugin:${plugin}|patch`, { id, state });
  };
}

/**
 * Removes the store from the collection and disposes of its resources.
 *
 * @internal
 */
export function unload(plugin: string) {
  return function (id: string): Promise<void> {
    return invoke(`plugin:${plugin}|unload`, { id });
  };
}
