<script lang="ts">
  import { openStore, store } from '$lib/store';
  import { exit } from '@tauri-apps/plugin-process';
  import { printStore } from '$lib/commands';
  import { clearAutosave, setAutosave } from '@tauri-store/svelte/src/lib/index.js';

  function onKeyDown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      void exit(0);
    }
  }
</script>

<svelte:window onkeydown={onKeyDown} />

<main>
  <div class="action">
    <button type="button" onclick={() => setAutosave(5000)}>Set Autosave</button>
    <button type="button" onclick={clearAutosave}>Clear Autosave</button>
  </div>

  <section id="counter">
    <p>
      Counter: {$store.counter}
      Nested: {$store.nested.foo.bar.baz}
    </p>
    <div class="action">
      <button type="button" onclick={() => $store.counter++}>Increment</button>
      <button type="button" onclick={() => $store.nested.foo.bar.baz++}>Increment Nested</button>
      <button type="button" onclick={() => store.start()}>Start</button>
      <button type="button" onclick={() => store.stop()}>Stop</button>
      <button type="button" onclick={() => store.save()}>Save</button>
      <button type="button" onclick={() => store.saveNow()}>Save Now</button>
      <button type="button" onclick={printStore}>Print</button>
      <button type="button" onclick={openStore}>Open</button>
    </div>
  </section>
</main>
