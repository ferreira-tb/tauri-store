import { tick } from 'svelte';
import * as commands from './commands';
import type { TauriPluginSvelteStoreOptions } from './types';
import {
  type Subscriber,
  type Unsubscriber,
  type Updater,
  writable,
  type Writable,
} from 'svelte/store';
import {
  BaseStore,
  type CustomStoreProperties,
  debounce,
  type State,
  throttle,
  TimeStrategy,
} from '@tauri-store/shared';

/**
 * Implementing `Writable<S>` certifies that our store follows the Svelte store contract.
 *
 * @see https://svelte.dev/docs/svelte/stores#Store-contract
 */
type StoreContract<S extends State> = CustomStoreProperties & Writable<S>;

export class Store<S extends State> extends BaseStore<S> implements StoreContract<S> {
  private readonly store: Writable<S>;
  protected override options: TauriPluginSvelteStoreOptions;
  protected override readonly flush = tick;

  constructor(
    public readonly id: string,
    state: S,
    options: TauriPluginSvelteStoreOptions = {}
  ) {
    super();

    this.store = writable(state);

    const saveStrategy = new TimeStrategy(options.saveStrategy, options.saveInterval);
    const syncStrategy = new TimeStrategy(options.syncStrategy, options.syncInterval);

    this.options = {
      onError: options.onError ?? console.error,
      saveOnChange: options.saveOnChange ?? false,
      saveInterval: saveStrategy.interval,
      saveStrategy: saveStrategy.strategy,
      syncInterval: syncStrategy.interval,
      syncStrategy: syncStrategy.strategy,
    };
  }

  public set(value: S) {
    this.store.set(value);
  }

  public update(updater: Updater<S>): void {
    this.store.update(updater);
  }

  public subscribe(run: Subscriber<S>, invalidate?: () => void): Unsubscriber {
    return this.store.subscribe(run, invalidate);
  }

  protected async load() {
    const state = await commands.load<S>(this.id);
    this.patchSelf(state);

    await this.flush();
    this.unwatch = this.watch();
  }

  protected async unload() {
    await commands.unload(this.id);
  }

  protected override watch() {
    const patchBackend = (value: S) => this.patchBackend(value);
    if (this.syncStrategy === 'debounce') {
      const fn = debounce(patchBackend, this.syncInterval);
      return this.store.subscribe(fn);
    } else if (this.syncStrategy === 'throttle') {
      const fn = throttle(patchBackend, this.syncInterval);
      return this.store.subscribe(fn);
    }

    return this.store.subscribe(patchBackend);
  }

  protected override patchSelf(state: S) {
    const _state = this.applyKeyFilters(state);
    this.store.update((value) => Object.assign(value, _state));
  }

  protected patchBackend(state: S) {
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

  public async getPath() {
    return commands.getStorePath(this.id);
  }

  public async save() {
    return commands.save(this.id);
  }

  public async saveAll() {
    return commands.saveAll();
  }
}
