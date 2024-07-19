import { invoke } from '@tauri-apps/api/core';

export function load(id: string): Promise<Record<string, unknown>> {
  return invoke('plugin:pinia|load', { id });
}

export function save(id: string): Promise<void> {
  return invoke('plugin:pinia|save', { id });
}

export function saveAll(): Promise<void> {
  return invoke('plugin:pinia|save_all');
}

export function set(id: string, state: Record<string, unknown>): Promise<void> {
  return invoke('plugin:pinia|set', { id, state });
}
