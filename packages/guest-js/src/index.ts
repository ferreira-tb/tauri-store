import { nextTick } from 'vue';
import * as commands from './commands';
import type { PiniaPluginContext } from 'pinia';
import type { MaybePromise } from '@tb-dev/utils';
import { type UnlistenFn, listen } from '@tauri-apps/api/event';

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
  onError?: (error: unknown) => MaybePromise<void>;
}

interface Payload {
  id: string;
  state: Record<string, unknown>;
}

// eslint-disable-next-line @typescript-eslint/naming-convention
export function TauriPluginPinia(ctx: PiniaPluginContext) {
  let enabled = false;
  let unsubscribe: (() => void) | undefined;
  let unlisten: UnlistenFn | undefined;

  const { onError = console.error } = ctx.options.tauri ?? {};

  const changeQueue: Payload[] = [];

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

  function subscribe() {
    return ctx.store.$subscribe((_, state) => {
      if (!enabled) return;
      commands.set(ctx.store.$id, state).catch(onError);
    });
  }

  async function processQueue() {
    while (changeQueue.length) {
      const payload = changeQueue.shift();
      if (payload && payload.id === ctx.store.$id) {
        unsubscribe?.();
        ctx.store.$patch(payload.state as any);

        await nextTick();

        unsubscribe?.();
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
          processQueue().catch(onError);
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
    $tauri: { start, stop }
  };
}
