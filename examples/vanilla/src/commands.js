import { invoke } from '@tauri-apps/api/core';

export function onError(err) {
  console.error(err);
  const message = err instanceof Error ? err.message : String(err);
  invoke('on_error', { message }).catch(console.error);
}

export function printStore() {
  invoke('print_store').catch(onError);
}
