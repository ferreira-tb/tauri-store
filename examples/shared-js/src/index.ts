import { invoke } from '@tauri-apps/api/core';

export function onError(err: unknown) {
  console.error(err);
  const message = err instanceof Error ? err.message : String(err);
  void invoke('on_error', { message });
}

export function onWarn(warn: unknown) {
  console.warn(warn);
  const message = warn instanceof Error ? warn.message : String(warn);
  void invoke('on_warn', { message });
}

export function printCounter() {
  void invoke('print_counter');
}
