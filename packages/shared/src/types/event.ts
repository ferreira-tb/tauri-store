import type { State, StoreBackendRawOptions } from './store';

/** @internal */
export type ConfigChangePayload = {
  config: StoreBackendRawOptions;
  id: string;
};

/** @internal */
export type StateChangePayload<T extends State = State> = {
  id: string;
  state: T;
};
