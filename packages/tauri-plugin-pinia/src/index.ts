import { nextTick } from 'vue';
import { applyOptions } from './state';
import * as commands from './commands';
import { watchDebounced } from '@vueuse/core';
import type { PiniaPluginContext } from 'pinia';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import type {
  Payload,
  State,
  TauriPluginPiniaOptions,
  TauriPluginPiniaStoreOptions,
} from './types';

export type * from './types';
export * from './commands/public';

const STORE_UPDATED = 'tauri-store://updated';

declare module 'pinia' {
  export interface PiniaCustomProperties {
    readonly $tauri: {
      /** Add the store to the sync denylist. */
      readonly disableSync: () => Promise<void>;
      /** Remove the store from the sync denylist. */
      readonly enableSync: () => Promise<void>;
      /** Path where the store is saved. */
      readonly getPath: () => Promise<string>;
      /** Save the store to the disk. */
      readonly save: () => Promise<void>;
      /** Start watching for changes. */
      readonly start: () => Promise<void>;
      /** Stop watching for changes. */
      readonly stop: () => void;
    };
  }

  export interface DefineStoreOptionsBase<S, Store> {
    readonly tauri?: TauriPluginPiniaStoreOptions;
  }
}

export function createPlugin(options: TauriPluginPiniaOptions = {}) {
  return function (ctx: PiniaPluginContext) {
    const {
      debounce = options.debounce ?? 0,
      deep = options.deep ?? true,
      onError = options.onError ?? console.error,
    } = ctx.options.tauri ?? options;

    const disableSync = () => commands.disableSync(ctx.store.$id);
    const enableSync = () => commands.enableSync(ctx.store.$id);
    const getPath = () => commands.getStorePath(ctx.store.$id);
    const save = () => commands.save(ctx.store.$id);

    let enabled = false;
    let changeQueue: Payload[] = [];
    let unsubscribe: (() => void) | undefined;
    let unlisten: (() => void) | undefined;

    async function start() {
      if (enabled) return;
      try {
        enabled = true;
        unsubscribe?.();
        await load();

        const webview = getCurrentWebviewWindow();
        const fn = await webview.listen<Payload>(STORE_UPDATED, ({ payload }) => {
          if (enabled && payload.id === ctx.store.$id) {
            changeQueue.push(payload);
            processChangeQueue().catch(onError);
          }
        });

        unlisten?.();
        unlisten = fn;
      } catch (err) {
        onError(err);
      }
    }

    async function load() {
      const state = await commands.load(ctx.store.$id);
      patchSelf(state);

      await nextTick();
      unsubscribe = subscribe();
    }

    function subscribe() {
      return watchDebounced(ctx.store.$state, patchBackend, { debounce, deep });
    }

    async function processChangeQueue() {
      while (changeQueue.length > 0) {
        await nextTick();
        const payload = changeQueue.pop();
        if (enabled && payload && payload.id === ctx.store.$id) {
          unsubscribe?.();
          patchSelf(payload.state);
          changeQueue = [];
          unsubscribe = subscribe();
        }
      }
    }

    async function stop() {
      try {
        unlisten?.();
        unsubscribe?.();
        enabled = false;
        await commands.unload(ctx.store.$id);
      } catch (err) {
        onError(err);
      }
    }

    function patchSelf(state: State) {
      const _state = applyOptions(state, ctx.options.tauri);
      ctx.store.$patch(_state as typeof ctx.store.$state);
    }

    function patchBackend(state: State) {
      if (enabled) {
        const _state = applyOptions(state, ctx.options.tauri);
        commands.patch(ctx.store.$id, _state).catch(onError);
      }
    }

    return {
      $tauri: {
        disableSync,
        enableSync,
        getPath,
        save,
        start,
        stop,
      },
    };
  };
}

// eslint-disable-next-line @typescript-eslint/naming-convention
export const TauriPluginPinia = createPlugin;
