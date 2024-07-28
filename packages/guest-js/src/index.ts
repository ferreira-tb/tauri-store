import { nextTick } from 'vue';
import * as commands from './commands';
import { watchDebounced } from '@vueuse/core';
import { listen } from '@tauri-apps/api/event';
import type { PiniaPluginContext } from 'pinia';
import type { MaybePromise } from '@tb-dev/utils';

export * from './commands';

const CHANGE_EVENT = 'pinia://change';

declare module 'pinia' {
  export interface PiniaCustomProperties {
    readonly $tauri: {
      readonly start: () => Promise<void>;
      readonly stop: () => void;
    };
  }

  export interface DefineStoreOptionsBase<S, Store> {
    readonly tauri?: TauriPluginPiniaOptions;
  }
}

export interface TauriPluginPiniaOptions {
  readonly debounce?: number;
  readonly onError?: (error: unknown) => MaybePromise<void>;
}

interface Payload {
  id: string;
  state: Record<string, unknown>;
}

export function createPlugin(options: TauriPluginPiniaOptions = {}) {
  return function (ctx: PiniaPluginContext) {
    let enabled = false;
    let unsubscribe: (() => void) | undefined;
    let unlisten: (() => void) | undefined;

    const { debounce = options.debounce ?? 0, onError = options.onError ?? console.error } =
      ctx.options.tauri ?? options;

    let changeQueue: Payload[] = [];

    async function load() {
      try {
        const storeState = await commands.load(ctx.store.$id);
        ctx.store.$patch(storeState as any);

        await nextTick();
        unsubscribe = subscribe();
      } catch (err) {
        onError(err);
      }
    }

    function patch(state: Record<string, unknown>) {
      if (enabled) {
        commands.patch(ctx.store.$id, state).catch(onError);
      }
    }

    function subscribe() {
      return watchDebounced(ctx.store.$state, patch, {
        debounce,
        deep: true,
      });
    }

    async function processChangeQueue() {
      while (changeQueue.length > 0) {
        await nextTick();
        const payload = changeQueue.pop();
        if (payload && payload.id === ctx.store.$id) {
          unsubscribe?.();
          ctx.store.$patch(payload.state as any);
          changeQueue = [];
          unsubscribe = subscribe();
        }
      }
    }

    async function start() {
      if (!enabled) {
        enabled = true;
        unsubscribe?.();
        await load();

        const fn = await listen<Payload>(CHANGE_EVENT, ({ payload }) => {
          if (payload.id === ctx.store.$id) {
            changeQueue.push(payload);
            processChangeQueue().catch(onError);
          }
        });

        unlisten?.();
        unlisten = fn;
      }
    }

    function stop() {
      unlisten?.();
      unsubscribe?.();
      enabled = false;
    }

    return {
      $tauri: { start, stop },
    };
  };
}
