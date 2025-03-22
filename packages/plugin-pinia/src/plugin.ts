import { Store } from './store';
import * as commands from './commands';
import type { PiniaPluginContext } from 'pinia';
import type { TauriPluginPiniaOptions } from './types';
import type { TauriStoreContract } from '@tauri-store/shared';

/**
 * Creates the Pinia plugin.
 *
 * This is also exported as [`TauriPluginPinia`].
 *
 * [`TauriPluginPinia`]: https://tb.dev.br/tauri-store/js-docs/plugin-pinia/functions/TauriPluginPinia.html
 *
 * @example
 * ```ts
 * import { createApp } from 'vue';
 * import { createPinia } from 'pinia';
 * import { createPlugin } from '@tauri-store/pinia';
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
  // eslint-disable-next-line @typescript-eslint/explicit-module-boundary-types
  return function (ctx: PiniaPluginContext) {
    const store = new Store(ctx, pluginOptions);
    const $tauri: TauriStoreContract = {
      getPath: () => commands.getStorePath(store.id),
      save: () => commands.save(store.id),
      saveAll: () => commands.saveAll(),
      saveAllNow: () => commands.saveAllNow(),
      saveNow: () => commands.save(store.id),
      start: () => store.start(),
      stop: () => store.stop(),
    };

    return { $tauri };
  };
}

/**
 * This is an alias for [`createPlugin`].
 *
 * [`createPlugin`]: https://tb.dev.br/tauri-store/js-docs/plugin-pinia/functions/createPlugin.html
 */
export const TauriPluginPinia = createPlugin;
