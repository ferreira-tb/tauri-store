<script setup lang="ts">
import { onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell';
import { saveAll } from 'tauri-plugin-pinia/src/index.ts';
import { useDebouncedStore, useStore, useThrottledStore } from './stores';

const store = useStore();
const { start, stop } = store.$tauri;

const debouncedStore = useDebouncedStore();
const { start: startDebounced, stop: stopDebounced } = debouncedStore.$tauri;

const throttledStore = useThrottledStore();
const { start: startThrottled, stop: stopThrottled } = throttledStore.$tauri;

function printCounter() {
  void invoke('print_counter');
}

async function openStore() {
  const path = await store.$tauri.getPath();
  await open(path);
}

async function openDebouncedStore() {
  const path = await debouncedStore.$tauri.getPath();
  await open(path);
}

async function openThrottledStore() {
  const path = await throttledStore.$tauri.getPath();
  await open(path);
}

onMounted(() => {
  void start();
  void startDebounced();
  void startThrottled();
});
</script>

<template>
  <main>
    <div class="action">
      <button type="button" @click="saveAll">Save All</button>
    </div>
    <section id="counter">
      <p>Counter: {{ store.counter }}</p>
      <div class="action">
        <button type="button" @click="store.increment">Increment</button>
        <button type="button" @click="start">Start</button>
        <button type="button" @click="stop">Stop</button>
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
        <button type="button" @click="openDebouncedStore">Open</button>
      </div>
    </section>

    <section id="throttled-counter">
      <p>Throttled Counter: {{ throttledStore.throttledCounter }}</p>
      <div class="action">
        <button type="button" @click="throttledStore.increment">Increment</button>
        <button type="button" @click="startThrottled">Start</button>
        <button type="button" @click="stopThrottled">Stop</button>
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
  justify-content: center;
  gap: 0.5rem;
}
</style>
