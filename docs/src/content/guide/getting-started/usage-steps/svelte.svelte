<script lang="ts">
  import type { Code } from '$lib/code';
  import { CodeBlock, CodeGroup } from '$components/code';
  import StartIt from '$content/guide/getting-started/start-it.svelte';

  const createStore: Code = {
    id: 'create-store',
    label: 'src/lib/stores/counter.ts',
    lang: 'typescript',
    value: `
import { Store } from 'tauri-plugin-svelte';

export const store = new Store('counter', { counter: 0 });
    `,
  };

  const startStore = `
import { store } from '$lib/stores/counter';

await store.start();
  `;

  // See: https://github.com/sveltejs/svelte/issues/6745
  const useStore = `
<script>
  import { store } from '$lib/stores/counter';
<\/script>

<div>
  <p>Counter: {$store.counter}</p>
  <button type="button" onclick={() => $store.counter++}>
    <span>Increment</span>
  </button>
</div>
`;
</script>

<li>
  <span>Create a store:</span>

  <CodeGroup code={createStore} />
</li>

<li>
  <span>Start it:</span>

  <CodeBlock lang="typescript" code={startStore} />

  <StartIt />
</li>

<li>
  <span>Use the store in your Svelte components:</span>

  <CodeBlock lang="svelte" code={useStore} />
</li>
