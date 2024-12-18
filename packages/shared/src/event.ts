import type { EventCallback, UnlistenFn } from '@tauri-apps/api/event';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

export enum StoreEvent {
  ConfigChange = 'tauri-store://config-change',
  StateChange = 'tauri-store://state-change',
  Unload = 'tauri-store://unload',
}

export function listen<T>(event: StoreEvent, listener: EventCallback<T>): Promise<UnlistenFn> {
  return getCurrentWebviewWindow().listen<T>(event, listener);
}
