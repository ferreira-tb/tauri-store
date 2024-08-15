import { invoke } from '@tauri-apps/api/core';

/** @internal */
export function load<T extends Record<string, unknown>>(id: string): Promise<T> {
  return invoke('plugin:pinia|load', { id });
}

/** @internal */
export function patch(id: string, state: Record<string, unknown>): Promise<void> {
  return invoke('plugin:pinia|patch', { id, state });
}

/** Save the store state to the disk. */
export function save(id: string): Promise<void> {
  return invoke('plugin:pinia|save', { id });
}

/** Save all store states to the disk. */
export function saveAll(): Promise<void> {
  return invoke('plugin:pinia|save_all');
}

/** @internal */
export function unload(id: string): Promise<void> {
  return invoke('plugin:pinia|unload', { id });
}
