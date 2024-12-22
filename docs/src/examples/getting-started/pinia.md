3. Enable the plugin for Pinia:

::: code-group

```ts{8} [src/index.ts]
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { createPlugin } from 'tauri-plugin-pinia';

const app = createApp(App);

const pinia = createPinia();
pinia.use(createPlugin());

app.use(pinia)
app.mount('#app');
```

:::

::: tip
`createPlugin` is also exported as `TauriPluginPinia`.
:::

4. Define your Pinia store:

::: code-group

```ts [src/stores/counter.ts]
import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useCounterStore = defineStore('counter', () => {
  const counter = ref(0);
  return { counter };
});
```

:::

5. Start the plugin:

```ts{4}
import { useCounterStore } from '@/stores/counter';

const counterStore = useCounterStore();
await counterStore.$tauri.start();
```

::: info
The stores won't be saved nor synchronized until you call the `start` method, but they can still be used as regular Pinia stores.
:::

### Nuxt

If you are using [Nuxt](https://nuxt.com/), you can create a [Nuxt plugin](https://nuxt.com/docs/guide/directory-structure/plugins) to enable it for Pinia:

::: code-group

```ts [plugins/pinia.ts]
import type { Pinia } from 'pinia';
import { TauriPluginPinia } from 'tauri-plugin-pinia';

// See: https://pinia.vuejs.org/core-concepts/plugins.html#Nuxt-js
export default defineNuxtPlugin(({ $pinia }) => {
  ($pinia as Pinia).use(TauriPluginPinia());
});
```

:::
