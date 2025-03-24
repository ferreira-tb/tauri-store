import { Store } from 'tauri-store/src/index.js';

export type State = {
  counter: number;
  version: number;
};

const initialState: State = {
  counter: 0,
  version: 0,
};

export const store = new Store('my-store', initialState);

if (!Object.hasOwn(window, '$store')) {
  Object.defineProperty(window, '$store', {
    value: store,
    writable: false,
    configurable: false,
    enumerable: true,
  });
}
