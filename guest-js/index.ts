import { nextTick } from 'vue';
import { invoke } from '@tauri-apps/api';
import { MutationType, type PiniaPluginContext } from 'pinia';

declare module 'pinia' {
  export interface PiniaCustomProperties {
    readonly $tauri: {
      readonly load: (path?: string) => Promise<void>;
      readonly save: () => Promise<void>;
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

class TauriStore {
  constructor(private readonly path: string) {}

  public async set(key: string, value: unknown) {
    await invoke('plugin:pinia|set', { path: this.path, key, value });
  }

  public async entries(): Promise<[string, unknown][]> {
    return invoke('plugin:pinia|entries', { path: this.path });
  }

  public async load() {
    await invoke('plugin:pinia|load', { path: this.path });
  }
}

export default function plugin(ctx: PiniaPluginContext) {
  let tauriStore: TauriStore | undefined;
  const options = ctx.options.tauri ?? {};

  let stop: (() => void) | undefined;
  async function load(path = options.path ?? `${ctx.store.$id}.pinia`) {
    tauriStore ??= new TauriStore(path);
    await tauriStore.load();

    const values = await tauriStore.entries().then((it) => {
      return it.reduce<Record<string, unknown>>((acc, [key, value]) => {
        if (Object.hasOwn(ctx.store, key)) {
          acc[key] = value;
        }

        return acc;
      }, {});
    });

    stop?.();

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    ctx.store.$patch(values as any);

    await nextTick();
    stop?.();

    stop = ctx.store.$subscribe((mutation, state) => {
      for (const [key, value] of Object.entries(state)) {
        if (mutation.type === MutationType.patchObject && !(key in mutation.payload)) {
          continue;
        }

        void tauriStore?.set(key, value);
      }
    });
  }

  return {
    $tauri: { load }
  };
}
