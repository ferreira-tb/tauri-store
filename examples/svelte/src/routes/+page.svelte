<script lang="ts">
  import { onMount } from 'svelte';
  import { exit } from '@tauri-apps/plugin-process';
  import { onError, printCounter } from 'example-shared-js/src/index.js';
  import {
    clearAutosave,
    saveAll,
    saveAllNow,
    setAutosave,
  } from 'tauri-plugin-svelte/src/index.js';
  import {
    debouncedStore,
    openDebouncedStore,
    openStore,
    openThrottledStore,
    store,
    throttledStore,
  } from '../stores';

  function onKeyDown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      exit(0).catch(onError);
    }
  }

  onMount(() => {
    // prettier-ignore
    store.start()
      .then(() => debouncedStore.start())
      .then(() => throttledStore.start())
      .catch(onError);
  });
</script>

<svelte:window onkeydown={onKeyDown} />

<main>
  <div class="action">
    <button type="button" onclick={saveAll}>Save All</button>
    <button type="button" onclick={saveAllNow}>Save All Now</button>
    <button type="button" onclick={() => setAutosave(5000)}>Set Autosave</button>
    <button type="button" onclick={clearAutosave}>Clear Autosave</button>
  </div>

  <section id="counter">
    <p>Counter: {$store.counter}</p>
    <div class="action">
      <button type="button" onclick={() => $store.counter++}>Increment</button>
      <button type="button" onclick={() => store.start()}>Start</button>
      <button type="button" onclick={() => store.stop()}>Stop</button>
      <button type="button" onclick={() => store.save()}>Save</button>
      <button type="button" onclick={() => store.saveNow()}>Save Now</button>
      <button type="button" onclick={printCounter}>Print</button>
      <button type="button" onclick={openStore}>Open</button>
    </div>
  </section>

  <section id="debounced-counter">
    <p>Debounced Counter: {$debouncedStore.debouncedCounter}</p>
    <div class="action">
      <button type="button" onclick={() => $debouncedStore.debouncedCounter++}>Increment</button>
      <button type="button" onclick={() => debouncedStore.start()}>Start</button>
      <button type="button" onclick={() => debouncedStore.stop()}>Stop</button>
      <button type="button" onclick={() => debouncedStore.save()}>Save</button>
      <button type="button" onclick={() => debouncedStore.saveNow()}>Save Now</button>
      <button type="button" onclick={openDebouncedStore}>Open</button>
    </div>
  </section>

  <section id="throttled-counter">
    <p>Throttled Counter: {$throttledStore.throttledCounter}</p>
    <div class="action">
      <button type="button" onclick={() => $throttledStore.throttledCounter++}>Increment</button>
      <button type="button" onclick={() => throttledStore.start()}>Start</button>
      <button type="button" onclick={() => throttledStore.stop()}>Stop</button>
      <button type="button" onclick={() => throttledStore.save()}>Save</button>
      <button type="button" onclick={() => throttledStore.saveNow()}>Save Now</button>
      <button type="button" onclick={openThrottledStore}>Open</button>
    </div>
  </section>
</main>
