import * as commands from './commands';
import type { PiniaPluginContext } from 'pinia';
import { nextTick, watch, type WatchOptions } from 'vue';
import type { TauriPluginPiniaOptions, TauriPluginPiniaStoreOptions } from './types';
import {
  BaseStore,
  debounce,
  mergeStoreOptions,
  type State,
  throttle,
  TimeStrategy,
} from '@tauri-store/shared';

export class Store extends BaseStore {
  protected readonly options: TauriPluginPiniaStoreOptions;
  protected override readonly flush = () => nextTick();

  constructor(
    private readonly ctx: PiniaPluginContext,
    pluginOptions: TauriPluginPiniaOptions
  ) {
    super();

    const options = mergeStoreOptions(ctx.options.tauri, pluginOptions);
    const saveStrategy = new TimeStrategy(options.saveStrategy, options.saveInterval);
    const syncStrategy = new TimeStrategy(options.syncStrategy, options.syncInterval);

    this.options = {
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

  protected async load() {
    const state = await commands.load(this.id);
    this.patchSelf(state);

    await this.flush();
    this.unwatch = this.watch();
  }

  protected async unload() {
    await commands.unload(this.id);
  }

  protected watch() {
    const patchBackend = this.patchBackend.bind(this);
    const options: WatchOptions = {
      deep: this.options.deep,
      flush: this.options.flush,
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

  protected async setOptions() {
    try {
      await commands.setStoreOptions(this.id, {
        saveInterval: this.options.saveInterval,
        saveStrategy: this.options.saveStrategy,
        saveOnChange: this.options.saveOnChange,
      });
    } catch (err) {
      this.onError?.(err);
    }
  }

  get id() {
    return this.ctx.store.$id;
  }
}
