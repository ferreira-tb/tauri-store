import { flushPromises } from './utils';
import type * as commands from './commands';
import { listen, StoreEvent } from './event';
import { type LooseTimeStrategyKind, TimeStrategy } from './time-strategy';
import { DEFAULT_AUTO_START, DEFAULT_FILTER_KEYS_STRATEGY } from './defaults';
import type {
  ConfigChangePayload,
  Option,
  State,
  StateChangePayload,
  StoreBackendRawOptions,
  StoreHooks,
  StoreKeyFilter,
  StoreKeyFilterStrategy,
  StoreOptions,
} from './types';

/**
 * Base class for the store implementations.
 *
 * @internal
 */
export abstract class BaseStore<S extends State = State> {
  public abstract readonly id: string;
  protected abstract readonly options: StoreOptions<S>;

  /** Whether the synchronization is enabled. */
  protected enabled = false;
  /** Queue of state changes to be processed. */
  protected changeQueue: StateChangePayload<S>[] = [];
  /** Flushes pending promises. */
  protected readonly flush = flushPromises;

  /** Starts the store synchronization. */
  public async start(): Promise<void> {
    if (this.enabled) return;
    try {
      this.enabled = true;
      this.unwatch?.();

      await this.load();
      await this.setOptions();

      const unlisten = await this.listen();
      this.unlisten?.();
      this.unlisten = unlisten;

      const unlistenOptions = await this.listenOptions();
      this.unlistenOptions?.();
      this.unlistenOptions = unlistenOptions;
    } catch (err) {
      if (this.onError) {
        await this.onError(err);
      } else {
        throw err;
      }
    }
  }

  /** Stops the store synchronization. */
  public async stop(): Promise<void> {
    if (!this.enabled) return;
    try {
      this.unlistenOptions?.();
      this.unlistenOptions = null;
      this.unlisten?.();
      this.unlisten = null;
      this.unwatch?.();
      this.unwatch = null;
      this.enabled = false;
      this.changeQueue = [];
      await this.unload();
    } catch (err) {
      if (this.onError) {
        await this.onError(err);
      } else {
        throw err;
      }
    }
  }

  /** Loads the store state from the backend. */
  protected abstract load(): Promise<void>;
  protected abstract unload(): Promise<void>;

  /** Watches itself for state changes, notifying the backend when necessary. */
  protected abstract watch(): () => void;
  /** Stops watching for changes in the store state. */
  protected unwatch: Option<() => void>;

  /** Listens for state changes coming from the backend. */
  protected listen(): Promise<() => void> {
    return listen<StateChangePayload<S>>(StoreEvent.StateChange, ({ payload }) => {
      if (this.enabled && payload.id === this.id) {
        this.changeQueue.push(payload);
        this.processChangeQueue().catch(async (err: unknown) => {
          if (this.onError) {
            await this.onError(err);
          } else {
            throw err;
          }
        });
      }
    });
  }

  /** Stops listening for state changes coming from the backend. */
  protected unlisten: Option<() => void>;

  private async listenOptions(): Promise<() => void> {
    return listen<ConfigChangePayload>(StoreEvent.ConfigChange, ({ payload }) => {
      if (this.enabled && payload.id === this.id) {
        this.patchOptions(payload.config);
      }
    });
  }

  private unlistenOptions: Option<() => void>;

  protected async processChangeQueue(): Promise<void> {
    while (this.changeQueue.length > 0) {
      await this.flush();
      const payload = this.changeQueue.pop();
      if (this.enabled && payload && payload.id === this.id) {
        this.unwatch?.();
        this.unwatch = null;
        this.patchSelf(payload.state);
        this.changeQueue = [];
        this.unwatch = this.watch();
      }
    }
  }

  protected abstract patchSelf(state: S): void;

  protected abstract patchBackend(state: S): void;

  protected patchBackendHelper(fn: ReturnType<typeof commands.patch>, state: S): void {
    if (this.enabled) {
      let _state = this.options.hooks?.beforeBackendSync
        ? this.options.hooks.beforeBackendSync(state)
        : state;

      if (_state) {
        _state = this.applyKeyFilters(_state);
        fn(this.id, _state).catch(async (err: unknown) => {
          if (this.onError) {
            await this.onError(err);
          } else {
            throw err;
          }
        });
      }
    }
  }

  protected abstract setOptions(): Promise<void>;

  protected async setOptionsHelper(fn: ReturnType<typeof commands.setStoreOptions>): Promise<void> {
    try {
      await fn(this.id, {
        saveInterval: this.options.saveInterval,
        saveOnChange: this.options.saveOnChange,
        saveOnExit: this.options.saveOnExit,
        saveStrategy: this.options.saveStrategy,
      });
    } catch (err) {
      if (this.onError) {
        await this.onError(err);
      } else {
        throw err;
      }
    }
  }

  private patchOptions(config: StoreBackendRawOptions): void {
    if (typeof config.saveOnChange === 'boolean') {
      this.options.saveOnChange = config.saveOnChange;
    }

    if (Array.isArray(config.saveStrategy)) {
      const saveStrategy = TimeStrategy.parse(config.saveStrategy);
      this.options.saveInterval = saveStrategy.interval;
      this.options.saveStrategy = saveStrategy.strategy;
    }
  }

  protected applyKeyFilters(state: Partial<S>): Partial<S> {
    const filter = this.options.filterKeys ?? null;
    const strategy = this.options.filterKeysStrategy ?? DEFAULT_FILTER_KEYS_STRATEGY;

    // If the strategy is a callback, `filterKeys` doesn't matter, as we won't match against it.
    if (!filter && typeof strategy !== 'function') {
      return state;
    }

    const result: Partial<S> = {};
    for (const [key, value] of Object.entries(state)) {
      if (shouldPickKey(filter, strategy, key)) {
        (result as State)[key] = value;
      }
    }

    return result;
  }

  // TODO: Don't you think we could give this a better name?
  protected async checkAutoStart(): Promise<void> {
    try {
      let autoStart = this.options.autoStart ?? DEFAULT_AUTO_START;
      if (typeof autoStart !== 'boolean') {
        autoStart = await autoStart(this.id);
      }

      if (autoStart) {
        await this.start();
      }
    } catch (err) {
      if (this.onError) {
        await this.onError(err);
      } else {
        throw err;
      }
    }
  }

  protected async updateDenylist(f: {
    allowSave: ReturnType<typeof commands.allowSave>;
    allowSync: ReturnType<typeof commands.allowSync>;
    denySave: ReturnType<typeof commands.denySave>;
    denySync: ReturnType<typeof commands.denySync>;
  }): Promise<void> {
    try {
      const promises: Promise<void>[] = [];
      if (this.options.save === false) {
        promises.push(f.denySave(this.id));
      } else {
        promises.push(f.allowSave(this.id));
      }

      if (this.options.sync === false) {
        promises.push(f.denySync(this.id));
      } else {
        promises.push(f.allowSync(this.id));
      }

      await Promise.all(promises);
    } catch (err) {
      if (this.onError) {
        await this.onError(err);
      } else {
        throw err;
      }
    }
  }

  /**
   * {@link StoreOptions.syncStrategy}
   */
  protected get syncStrategy(): LooseTimeStrategyKind {
    return this.options.syncStrategy;
  }

  /**
   * {@link StoreOptions.saveStrategy}
   */
  protected get syncInterval(): Option<number> {
    return this.options.syncInterval;
  }

  /**
   * {@link StoreHooks.error}
   */
  protected get onError(): Option<StoreHooks<S>['error']> {
    return this.options.hooks?.error;
  }
}

function shouldPickKey(
  filter: StoreKeyFilter,
  strategy: StoreKeyFilterStrategy,
  key: string
): boolean {
  return (
    (strategy === 'pick' && isKeyMatch(filter, key)) ||
    (strategy === 'omit' && !isKeyMatch(filter, key)) ||
    (typeof strategy === 'function' && strategy(key))
  );
}

function isKeyMatch(filter: StoreKeyFilter, key: string): boolean {
  return (
    (typeof filter === 'string' && key === filter) ||
    (Array.isArray(filter) && filter.includes(key)) ||
    (filter instanceof RegExp && filter.test(key))
  );
}
