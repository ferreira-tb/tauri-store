import type { MaybePromise } from '@tb-dev/utils';

export interface TauriPluginPiniaOptions {
  /** @default 0 */
  readonly debounce?: number;
  /** @default true */
  readonly deep?: boolean;

  /**
   * Custom error handler.
   * @default console.error
   */
  readonly onError?: (error: unknown) => MaybePromise<void>;
}

export interface TauriPluginPiniaStoreOptions extends TauriPluginPiniaOptions {
  /** Denylist of keys the plugin should ignore. Those won't be saved nor synced. */
  readonly ignoreKey?: string | string[];
}

export type State = Record<string, unknown>;

export interface Payload {
  id: string;
  state: State;
}
