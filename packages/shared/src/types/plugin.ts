import type { MaybePromise } from '@tb-dev/utils';
import type { LooseTimeStrategyKind } from '../utils/time-strategy';

export interface PluginOptions {
  /**
   * Custom error handler.
   * @default console.error
   */
  readonly onError?: (error: unknown) => MaybePromise<void>;

  /**
   * Interval in milliseconds to use when syncing the store with the backend.
   * This option is only valid when {@link PluginOptions.syncStrategy} is set to `debounce` or `throttle`.
   *
   * @default 0
   */
  readonly syncInterval?: number;

  /**
   * Strategy to use when syncing the store with the backend.
   *
   * Whenever there's a state change, the store sends a notification to Rust so they can stay in sync.
   * Since data gets serialized every time this happens, it can be expensive to do it too often.
   * To mitigate this, we can use `debounce` or `throttle` to control how often this synchronization occurs.
   *
   * If the value is a number, the plugin will use `debounce` with the specified value as the interval.
   * If `null` or `undefined`, the strategy will be `immediate`.
   *
   * For a detailed explanation about the differences between `debounce` and `throttle`, see:
   * https://kettanaito.com/blog/debounce-vs-throttle
   *
   * @default 'immediate'
   */
  readonly syncStrategy?: LooseTimeStrategyKind;
}
