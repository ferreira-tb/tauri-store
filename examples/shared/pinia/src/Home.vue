<script setup lang="ts">
import { onMounted } from 'vue';
import { onKeyDown } from '@vueuse/core';
import { exit } from '@tauri-apps/plugin-process';
import { onError, printCounter } from 'example-shared-js/src/index.js';
import {
  clearAutosave,
  save,
  saveAll,
  saveAllNow,
  saveNow,
  setAutosave,
} from 'tauri-plugin-pinia/src/index.js';
import {
  openDebouncedStore,
  openStore,
  openThrottledStore,
  useDebouncedStore,
  useStore,
  useThrottledStore,
} from './stores';

const store = useStore();
const { start, stop } = store.$tauri;

const debouncedStore = useDebouncedStore();
const { start: startDebounced, stop: stopDebounced } = debouncedStore.$tauri;

const throttledStore = useThrottledStore();
const { start: startThrottled, stop: stopThrottled } = throttledStore.$tauri;

onKeyDown('Escape', () => void exit(0));

onMounted(() => {
  // prettier-ignore
  start()
    .then(() => startDebounced())
    .then(() => startThrottled())
    .catch(onError);
});
</script>

<template>
  <main>
    <div class="action">
      <button type="button" @click="saveAll">Save All</button>
      <button type="button" @click="saveAllNow">Save All Now</button>
      <button type="button" @click="() => setAutosave(5000)">Set Autosave</button>
      <button type="button" @click="clearAutosave">Clear Autosave</button>
    </div>
    <section id="counter">
      <p>Counter: {{ store.counter }}</p>
      <div class="action">
        <button type="button" @click="store.increment">Increment</button>
        <button type="button" @click="start">Start</button>
        <button type="button" @click="stop">Stop</button>
        <button type="button" @click="save(store.$id)">Save</button>
        <button type="button" @click="saveNow(store.$id)">Save Now</button>
        <button type="button" @click="printCounter">Print</button>
        <button type="button" @click="openStore">Open</button>
      </div>
    </section>

    <section id="debounced-counter">
      <p>Debounced Counter: {{ debouncedStore.debouncedCounter }}</p>
      <div class="action">
        <button type="button" @click="debouncedStore.increment">Increment</button>
        <button type="button" @click="startDebounced">Start</button>
        <button type="button" @click="stopDebounced">Stop</button>
        <button type="button" @click="save(debouncedStore.$id)">Save</button>
        <button type="button" @click="saveNow(debouncedStore.$id)">Save Now</button>
        <button type="button" @click="openDebouncedStore">Open</button>
      </div>
    </section>

    <section id="throttled-counter">
      <p>Throttled Counter: {{ throttledStore.throttledCounter }}</p>
      <div class="action">
        <button type="button" @click="throttledStore.increment">Increment</button>
        <button type="button" @click="startThrottled">Start</button>
        <button type="button" @click="stopThrottled">Stop</button>
        <button type="button" @click="save(throttledStore.$id)">Save</button>
        <button type="button" @click="saveNow(throttledStore.$id)">Save Now</button>
        <button type="button" @click="openThrottledStore">Open</button>
      </div>
    </section>
  </main>
</template>

<style scoped>
main,
section {
  display: flex;
  flex-direction: column;
}

section {
  gap: 0.25rem;
  margin-bottom: 0.5rem;
}

.action {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  justify-content: center;
  gap: 0.5rem;
}
</style>
