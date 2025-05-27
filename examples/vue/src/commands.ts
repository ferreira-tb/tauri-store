import { invoke } from '@tauri-apps/api/core';

export function printStore() {
  return invoke('print_store');
}
