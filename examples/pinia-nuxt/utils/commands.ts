import { invoke } from '@tauri-apps/api/core';

export function onError(err: unknown) {
  console.error(err);
  const message = err instanceof Error ? err.message : String(err);
  void invoke('on_error', { message });
}

export function printStore() {
  void invoke('print_store');
}
