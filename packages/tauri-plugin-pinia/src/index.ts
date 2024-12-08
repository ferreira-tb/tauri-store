import * as commands from './commands';
import type { PiniaPluginContext } from 'pinia';
import { nextTick, watch, type WatchOptions } from 'vue';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import type { TauriPluginPiniaOptions, TauriPluginPiniaStoreOptions } from './types';
import {
  type ChangePayload,
  debounce,
  isStoreKeyMatch,
  type State,
  STORE_UPDATED,
  throttle,
} from '@tauri-store/shared';

export type * from './types';
export * from './commands/public';

declare module 'pinia' {
  export interface PiniaCustomProperties {
    readonly $tauri: {
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

class Plugin {
  private readonly deep: NonNullable<TauriPluginPiniaOptions['deep']>;
  private readonly flush: NonNullable<TauriPluginPiniaOptions['flush']>;
  private readonly onError: NonNullable<TauriPluginPiniaOptions['onError']>;

  private readonly syncInterval: TauriPluginPiniaOptions['syncInterval'];
  private readonly syncStrategy: TauriPluginPiniaOptions['syncStrategy'];

  private enabled = false;
  private changeQueue: ChangePayload[] = [];
  private unsubscribe: (() => void) | undefined;
  private unlisten: (() => void) | undefined;

  constructor(
    private readonly ctx: PiniaPluginContext,
    options: TauriPluginPiniaOptions = {}
  ) {
    const {
      deep = options.deep ?? true,
      flush = options.flush ?? 'pre',
      onError = options.onError ?? console.error,
      syncInterval = options.syncInterval ?? 0,
      syncStrategy = options.syncStrategy ?? 'immediate',
    } = ctx.options.tauri ?? options;

    this.deep = deep;
    this.flush = flush;
    this.onError = onError;
    this.syncInterval = syncInterval;
    this.syncStrategy = syncStrategy;

    if (typeof this.syncStrategy === 'number') {
      if (Number.isFinite(this.syncStrategy) && this.syncStrategy > 0) {
        if (!Number.isFinite(this.syncInterval) || this.syncInterval <= 0) {
          this.syncInterval = this.syncStrategy;
        }

        this.syncStrategy = 'debounce';
      } else {
        this.syncStrategy = 'immediate';
      }
    }
  }

  public async start() {
    if (this.enabled) return;
    try {
      this.enabled = true;
      this.unsubscribe?.();
      await this.load();

      const webview = getCurrentWebviewWindow();
      const fn = await webview.listen<ChangePayload>(STORE_UPDATED, ({ payload }) => {
        if (this.enabled && payload.id === this.id) {
          this.changeQueue.push(payload);
          this.processChangeQueue().catch(this.onError);
        }
      });

      this.unlisten?.();
      this.unlisten = fn;
    } catch (err) {
      this.onError(err);
    }
  }

  private async load() {
    const state = await commands.load(this.id);
    this.patchSelf(state);

    await nextTick();
    this.unsubscribe = this.subscribe();
  }

  private subscribe() {
    const patchBackend = this.patchBackend.bind(this);
    const options: WatchOptions = { deep: this.deep, flush: this.flush };

    if (this.syncStrategy === 'debounce') {
      const fn = debounce(patchBackend, this.syncInterval);
      return watch(this.ctx.store.$state, fn, options);
    } else if (this.syncStrategy === 'throttle') {
      const fn = throttle(patchBackend, this.syncInterval);
      return watch(this.ctx.store.$state, fn, options);
    }

    return watch(this.ctx.store.$state, patchBackend, options);
  }

  private async processChangeQueue() {
    while (this.changeQueue.length > 0) {
      await nextTick();
      const payload = this.changeQueue.pop();
      if (this.enabled && payload && payload.id === this.id) {
        this.unsubscribe?.();
        this.patchSelf(payload.state);
        this.changeQueue = [];
        this.unsubscribe = this.subscribe();
      }
    }
  }

  private patchSelf(state: State) {
    const _state = this.applyKeyFilters(state);
    this.ctx.store.$patch(_state as typeof this.ctx.store.$state);
  }

  private patchBackend(state: State) {
    if (this.enabled) {
      const _state = this.applyKeyFilters(state);
      commands.patch(this.id, _state).catch(this.onError);
    }
  }

  private applyKeyFilters(state: State) {
    const keys = this.storeOptions?.filterKeys;
    const strategy = this.storeOptions?.filterKeysStrategy ?? 'omit';

    if (keys) {
      const result: State = {};
      for (const [key, value] of Object.entries(state)) {
        if (
          (strategy === 'omit' && isStoreKeyMatch(keys, key)) ||
          (strategy === 'pick' && !isStoreKeyMatch(keys, key)) ||
          (typeof strategy === 'function' && !strategy(key))
        ) {
          continue;
        }

        result[key] = value;
      }

      return result;
    }

    return state;
  }

  public async stop() {
    try {
      this.unlisten?.();
      this.unsubscribe?.();
      this.enabled = false;
      await commands.unload(this.id);
    } catch (err) {
      this.onError(err);
    }
  }

  public getPath() {
    return commands.getStorePath(this.id);
  }

  public save() {
    return commands.save(this.id);
  }

  get id() {
    return this.ctx.store.$id;
  }

  private get storeOptions() {
    return this.ctx.options.tauri;
  }
}

export function createPlugin(options: TauriPluginPiniaOptions = {}) {
  return function (ctx: PiniaPluginContext) {
    const plugin = new Plugin(ctx, options);
    return {
      $tauri: {
        getPath: plugin.getPath.bind(plugin),
        save: plugin.save.bind(plugin),
        start: plugin.start.bind(plugin),
        stop: plugin.stop.bind(plugin),
      },
    };
  };
}

// eslint-disable-next-line @typescript-eslint/naming-convention
export const TauriPluginPinia = createPlugin;
