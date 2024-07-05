import type { PiniaPluginContext } from 'pinia';
import { Store as TauriStore } from '@tauri-apps/plugin-store';

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
  /** @default true */
  lazy?: boolean;
  /** @default Store id + `.pinia` */
  path?: string;
}

export default function plugin(ctx: PiniaPluginContext) {
  let tauriStore: TauriStore | undefined;
  const options = ctx.options.tauri ?? {};

  if (options.lazy === false) {
    void load();
  }

  ctx.store.$subscribe((_mutation, state) => {
    for (const [key, value] of Object.entries(state)) {
      void tauriStore?.set(key, value);
    }
  });

  async function load(path = options.path ?? `${ctx.store.$id}.pinia`) {
    tauriStore ??= new TauriStore(path);
    await tauriStore.load();
    await patch();
  }

  async function save() {
    await tauriStore?.save();
  }

  async function patch(initialValue: Record<string, unknown> = {}) {
    if (!tauriStore) return;
    const state = await tauriStore.entries().then((it) => {
      return it.reduce((acc, [key, value]) => {
        if (Object.hasOwn(ctx.store, key)) {
          acc[key] = value;
        }

        return acc;
      }, initialValue);
    });

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    ctx.store.$patch(state as any);
  }

  return {
    $tauri: { load, save }
  };
}
