import { listen, StoreEvent } from './event';
import { TimeStrategy } from './utils/time-strategy';
import type { Option, Writeable } from '@tb-dev/utils';
import { DEFAULT_FILTER_KEYS_STRATEGY } from './defaults';
import type {
  ConfigChangePayload,
  State,
  StateChangePayload,
  StoreBackendRawOptions,
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
  protected abstract readonly options: StoreOptions;

  /** Whether the synchronization is enabled. */
  protected enabled = false;
  /** Queue of state changes to be processed. */
  protected changeQueue: StateChangePayload<S>[] = [];
  /** Flushes pending promises. */
  protected readonly flush = flushPromises;

  /** Starts the store synchronization. */
  public async start() {
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
      this.onError?.(err);
    }
  }

  /** Stops the store synchronization. */
  public async stop() {
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
      this.onError?.(err);
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
        this.processChangeQueue().catch((err) => this.onError?.(err));
      }
    });
  }

  /** Stops listening for state changes coming from the backend. */
  protected unlisten: Option<() => void>;

  private async listenOptions() {
    return listen<ConfigChangePayload>(StoreEvent.ConfigChange, ({ payload }) => {
      if (this.enabled && payload.id === this.id) {
        this.patchOptions(payload.config);
      }
    });
  }

  private unlistenOptions: Option<() => void>;

  protected async processChangeQueue() {
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

  protected abstract setOptions(): Promise<void>;

  private patchOptions(config: StoreBackendRawOptions) {
    if (typeof config.saveOnChange === 'boolean') {
      this.options.saveOnChange = config.saveOnChange;
    }

    if (Array.isArray(config.saveStrategy)) {
      const saveStrategy = TimeStrategy.parse(config.saveStrategy);
      this.options.saveInterval = saveStrategy.interval;
      this.options.saveStrategy = saveStrategy.strategy;
    }
  }

  protected applyKeyFilters(state: S) {
    if (!this.options.filterKeys) {
      return state;
    }

    const result: Partial<S> = {};
    const filter = this.options.filterKeys;
    const strategy = this.options.filterKeysStrategy ?? DEFAULT_FILTER_KEYS_STRATEGY;

    for (const [key, value] of Object.entries(state)) {
      if (!shouldFilterKey(filter, strategy, key)) {
        (result as State)[key] = value;
      }
    }

    return result;
  }

  /**
   * {@link StoreOptions.syncStrategy}
   */
  protected get syncStrategy() {
    return this.options.syncStrategy;
  }

  /**
   * {@link StoreOptions.saveStrategy}
   */
  protected get syncInterval() {
    return this.options.syncInterval;
  }

  /**
   * {@link StoreOptions.onError}
   */
  protected get onError() {
    return this.options.onError;
  }
}

/**
 * @internal
 */
export function mergeStoreOptions<T extends StoreOptions, U extends StoreOptions>(
  target: T = {} as T,
  source: U = {} as U
): T & U {
  type Key = keyof StoreOptions;
  for (const [key, value] of Object.entries(source)) {
    if (typeof target[key as Key] === 'undefined') {
      (target as Writeable<StoreOptions>)[key as Key] = value;
    }
  }

  return target as T & U;
}

function shouldFilterKey(
  filter: StoreKeyFilter,
  strategy: StoreKeyFilterStrategy,
  key: string
): boolean {
  return (
    (strategy === 'omit' && isStoreKeyMatch(filter, key)) ||
    (strategy === 'pick' && !isStoreKeyMatch(filter, key)) ||
    (typeof strategy === 'function' && !strategy(key))
  );
}

function isStoreKeyMatch(filter: StoreKeyFilter, key: string): boolean {
  return (
    (typeof filter === 'string' && key === filter) ||
    (Array.isArray(filter) && filter.includes(key)) ||
    (filter instanceof RegExp && filter.test(key))
  );
}

function flushPromises(): Promise<void> {
  return new Promise((resolve) => void setTimeout(resolve, 0));
}
