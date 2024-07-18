import { nextTick } from 'vue';
import { invoke } from '@tauri-apps/api';
import { MutationType, type PiniaPluginContext } from 'pinia';

declare module 'pinia' {
  export interface PiniaCustomProperties {
    readonly $tauri: {
      readonly load: (path?: string) => Promise<void>;
      readonly start: () => Promise<void>;
      readonly stop: () => void;
    };
  }

  export interface DefineStoreOptionsBase<S, Store> {
    readonly tauri?: TauriPluginPiniaOptions;
  }
}

export interface TauriPluginPiniaOptions {
  /** @default Store id + `.pinia` */
  path?: string;
}

interface TauriStore {
  entries: () => Promise<[string, unknown][]>;
  load: () => Promise<void>;
  set: (key: string, value: unknown) => Promise<void>;
}

// eslint-disable-next-line @typescript-eslint/naming-convention
export function TauriPluginPinia(ctx: PiniaPluginContext) {
  let enabled = false;
  let tauriStore: TauriStore | undefined;
  const options = ctx.options.tauri ?? {};

  let stopWatcher: (() => void) | undefined;
  async function load(path = options.path ?? `${ctx.store.$id}.pinia`) {
    if (!enabled) return;

    tauriStore ??= {
      async entries(): Promise<[string, unknown][]> {
        return invoke('plugin:pinia|entries', { path });
      },
      async load() {
        await invoke('plugin:pinia|load', { path });
      },
      async set(key: string, value: unknown) {
        await invoke('plugin:pinia|set', { path, key, value });
      }
    };

    await tauriStore.load();

    const values = await tauriStore.entries().then((it) => {
      return it.reduce<Record<string, unknown>>((acc, [key, value]) => {
        if (Object.hasOwn(ctx.store, key)) {
          acc[key] = value;
        }

        return acc;
      }, {});
    });

    stopWatcher?.();

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    ctx.store.$patch(values as any);

    await nextTick();
    stopWatcher?.();

    stopWatcher = ctx.store.$subscribe((mutation, state) => {
      if (!enabled) return;
      for (const [key, value] of Object.entries(state)) {
        if (mutation.type === MutationType.patchObject && !(key in mutation.payload)) {
          continue;
        }

        void tauriStore?.set(key, value);
      }
    });
  }

  async function start() {
    if (!enabled) {
      enabled = true;
      await load();
    }
  }

  function stop() {
    stopWatcher?.();
    enabled = false;
  }

  return {
    $tauri: { load, start, stop }
  };
}
