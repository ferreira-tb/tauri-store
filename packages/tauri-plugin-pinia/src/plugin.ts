import * as commands from './commands';
import type { Option } from '@tb-dev/utils';
import type { PiniaPluginContext } from 'pinia';
import { nextTick, watch, type WatchOptions } from 'vue';
import type { TauriPluginPiniaOptions, TauriPluginPiniaStoreOptions } from './types';
import {
  BasePlugin,
  type CustomStoreProperties,
  debounce,
  listen,
  mergePluginOptions,
  type State,
  type StateChangePayload,
  StoreEvent,
  throttle,
  TimeStrategy,
} from '@tauri-store/shared';

class Plugin extends BasePlugin {
  protected readonly storeOptions: TauriPluginPiniaStoreOptions;

  private unsubscribe: Option<() => void>;
  private unlisten: Option<() => void>;

  constructor(
    private readonly ctx: PiniaPluginContext,
    pluginOptions: TauriPluginPiniaOptions
  ) {
    super();

    const options = mergePluginOptions(ctx.options.tauri, pluginOptions);
    const saveStrategy = new TimeStrategy(options.saveStrategy, options.saveInterval);
    const syncStrategy = new TimeStrategy(options.syncStrategy, options.syncInterval);

    this.storeOptions = {
      deep: options.deep ?? true,
      flush: options.flush ?? 'pre',
      onError: options.onError ?? console.error,
      saveOnChange: options.saveOnChange ?? false,
      saveInterval: saveStrategy.interval,
      saveStrategy: saveStrategy.strategy,
      syncInterval: syncStrategy.interval,
      syncStrategy: syncStrategy.strategy,
    };
  }

  public async start() {
    if (this.enabled) return;
    try {
      this.enabled = true;
      this.unsubscribe?.();

      await this.load();
      await this.listenConfigChanges();
      await this.setStoreOptions();

      const fn = await listen<StateChangePayload>(StoreEvent.StateChange, ({ payload }) => {
        if (this.enabled && payload.id === this.id) {
          this.changeQueue.push(payload);
          this.processChangeQueue().catch((err) => this.onError?.(err));
        }
      });

      this.unlisten?.();
      this.unlisten = fn;
    } catch (err) {
      this.onError?.(err);
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
    const options: WatchOptions = {
      deep: this.storeOptions.deep,
      flush: this.storeOptions.flush,
    };

    if (this.syncStrategy === 'debounce') {
      const fn = debounce(patchBackend, this.syncInterval);
      return watch(this.ctx.store.$state, fn, options);
    } else if (this.syncStrategy === 'throttle') {
      const fn = throttle(patchBackend, this.syncInterval);
      return watch(this.ctx.store.$state, fn, options);
    }

    return watch(this.ctx.store.$state, patchBackend, options);
  }

  protected async processChangeQueue() {
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

  protected patchSelf(state: State) {
    const _state = this.applyKeyFilters(state);
    this.ctx.store.$patch(_state as typeof this.ctx.store.$state);
  }

  protected patchBackend(state: State) {
    if (this.enabled) {
      const _state = this.applyKeyFilters(state);
      commands.patch(this.id, _state).catch((err) => this.onError?.(err));
    }
  }

  protected async setStoreOptions() {
    try {
      await commands.setStoreOptions(this.id, {
        saveInterval: this.storeOptions.saveInterval,
        saveStrategy: this.storeOptions.saveStrategy,
        saveOnChange: this.storeOptions.saveOnChange,
      });
    } catch (err) {
      this.onError?.(err);
    }
  }

  public async stop() {
    try {
      this.unlisten?.();
      this.unsubscribe?.();
      this.enabled = false;
      this.changeQueue = [];
      await commands.unload(this.id);
    } catch (err) {
      this.onError?.(err);
    }
  }

  get id() {
    return this.ctx.store.$id;
  }
}

/**
 * Creates the Pinia plugin.
 *
 * This is also exported as
 * [`TauriPluginPinia`](https://tb.dev.br/tauri-store/reference/tauri-plugin-pinia/functions/TauriPluginPinia.html).
 *
 * @example
 * ```ts
 * import { createApp } from 'vue';
 * import { createPinia } from 'pinia';
 * import { createPlugin } from 'tauri-plugin-pinia';
 *
 * const app = createApp(App);
 *
 * const pinia = createPinia();
 * pinia.use(createPlugin());
 *
 * app.use(pinia)
 * app.mount('#app');
 * ```
 */
export function createPlugin(pluginOptions: TauriPluginPiniaOptions = {}) {
  return function (ctx: PiniaPluginContext) {
    const plugin = new Plugin(ctx, pluginOptions);
    const $tauri: CustomStoreProperties = {
      getPath: () => commands.getStorePath(plugin.id),
      save: () => commands.save(plugin.id),
      saveAll: () => commands.saveAll(),
      start: () => plugin.start(),
      stop: () => plugin.stop(),
    };

    return { $tauri };
  };
}

/**
 * Creates the Pinia plugin.
 *
 * This is an alias for
 * [`createPlugin`](https://tb.dev.br/tauri-store/reference/tauri-plugin-pinia/functions/createPlugin.html).
 */
// eslint-disable-next-line @typescript-eslint/naming-convention
export const TauriPluginPinia = createPlugin;
