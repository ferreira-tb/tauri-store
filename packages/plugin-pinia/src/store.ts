import * as commands from './commands';
import type { PiniaPluginContext } from 'pinia';
import { nextTick, watch, type WatchOptions } from 'vue';
import type { TauriPluginPiniaOptions, TauriPluginPiniaStoreOptions } from './types';
import {
  BaseStore,
  debounce,
  DEFAULT_FILTER_KEYS,
  DEFAULT_FILTER_KEYS_STRATEGY,
  DEFAULT_HOOKS,
  DEFAULT_SAVE_ON_CHANGE,
  DEFAULT_SAVE_ON_EXIT,
  type Fn,
  merge,
  type State,
  throttle,
  TimeStrategy,
} from '@tauri-store/shared';

export class Store extends BaseStore {
  protected readonly options: TauriPluginPiniaStoreOptions;
  protected override readonly flush = (): Promise<void> => nextTick();

  constructor(
    private readonly ctx: PiniaPluginContext,
    pluginOptions: TauriPluginPiniaOptions
  ) {
    super();

    const options = merge<TauriPluginPiniaStoreOptions>(ctx.options.tauri, pluginOptions);
    const saveStrategy = new TimeStrategy(options.saveStrategy, options.saveInterval);
    const syncStrategy = new TimeStrategy(options.syncStrategy, options.syncInterval);

    this.options = {
      deep: options.deep ?? true,
      filterKeys: options.filterKeys ?? DEFAULT_FILTER_KEYS,
      filterKeysStrategy: options.filterKeysStrategy ?? DEFAULT_FILTER_KEYS_STRATEGY,
      flush: options.flush ?? 'pre',
      hooks: merge(options.hooks, DEFAULT_HOOKS),
      saveInterval: saveStrategy.interval,
      saveOnChange: options.saveOnChange ?? DEFAULT_SAVE_ON_CHANGE,
      saveOnExit: options.saveOnExit ?? DEFAULT_SAVE_ON_EXIT,
      saveStrategy: saveStrategy.strategy,
      syncInterval: syncStrategy.interval,
      syncStrategy: syncStrategy.strategy,
    } satisfies Required<TauriPluginPiniaStoreOptions>;
  }

  protected async load(): Promise<void> {
    const state = await commands.load(this.id);
    this.patchSelf(state);

    await this.flush();
    this.unwatch = this.watch();
  }

  protected async unload(): Promise<void> {
    await commands.unload(this.id);
  }

  protected watch(): Fn {
    let patchBackend = (state: State) => {
      this.patchBackend({ ...state });
    };

    const options: WatchOptions = {
      deep: this.options.deep,
      flush: this.options.flush,
    };

    if (this.syncStrategy === 'debounce') {
      patchBackend = debounce(patchBackend, this.syncInterval);
    } else if (this.syncStrategy === 'throttle') {
      patchBackend = throttle(patchBackend, this.syncInterval);
    }

    return watch(this.ctx.store.$state, patchBackend, options);
  }

  protected patchSelf(state: State): void {
    let _state = this.options.hooks?.beforeFrontendSync
      ? this.options.hooks.beforeFrontendSync(state)
      : state;

    if (_state) {
      _state = this.applyKeyFilters(_state);
      this.ctx.store.$patch(_state as typeof this.ctx.store.$state);
    }
  }

  protected patchBackend(state: State): void {
    this.patchBackendHelper(commands.patch, state);
  }

  protected async setOptions(): Promise<void> {
    return this.setOptionsHelper(commands.setStoreOptions);
  }

  get id(): string {
    return this.ctx.store.$id;
  }
}
