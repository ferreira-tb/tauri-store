import { invoke } from '@tauri-apps/api/core';

export function load(id: string): Promise<Record<string, unknown>> {
  return invoke('plugin:pinia|load', { id });
}

export function patch(id: string, state: Record<string, unknown>): Promise<void> {
  return invoke('plugin:pinia|patch', { id, state });
}

export function save(id: string): Promise<void> {
  return invoke('plugin:pinia|save', { id });
}

export function saveAll(): Promise<void> {
  return invoke('plugin:pinia|save_all');
}
