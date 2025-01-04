import UsageSteps from './index.svelte';

export { UsageSteps };

const useStore = `
<script>
  import { counterStore } from '$lib/stores/counter';
</script>

<div>
  <p>Counter: {$counterStore.counter}</p>
  <button type="button" onclick={() => $counterStore.counter++}>
    <span>Increment</span>
  </button>
</div>
`;
