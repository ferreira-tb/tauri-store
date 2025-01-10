<script lang="ts">
  import type { Code } from '$lib/code';
  import { CodeBlock, CodeGroup } from '$lib/components/code';
  import StartIt from '$lib/content/guide/getting-started/start-it.svelte';

  const createStore: Code = {
    id: 'create-store',
    label: 'src/lib/stores/counter.ts',
    lang: 'typescript',
    value: `
import { Store } from 'tauri-plugin-svelte';

export const counterStore = new Store('counter', { counter: 0 });
    `,
  };

  const startStore = `
import { counterStore } from '$lib/stores/counter';

await counterStore.start();
  `;

  // See: https://github.com/sveltejs/svelte/issues/6745
  const useStore = `
<script>
  import { counterStore } from '$lib/stores/counter';
<\/script>

<div>
  <p>Counter: {$counterStore.counter}</p>
  <button type="button" onclick={() => $counterStore.counter++}>
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
