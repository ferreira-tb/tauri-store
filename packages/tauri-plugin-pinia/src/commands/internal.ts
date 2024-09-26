import type { State } from '../types';
import { invoke } from '@tauri-apps/api/core';

/** @internal */
export function load<T extends State>(id: string): Promise<T> {
  return invoke('plugin:pinia|load', { id });
}

/** @internal */
export function patch(id: string, state: State): Promise<void> {
  return invoke('plugin:pinia|patch', { id, state });
}

/** @internal */
export function unload(id: string): Promise<void> {
  return invoke('plugin:pinia|unload', { id });
}
