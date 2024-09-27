<script setup lang="ts">
import { onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell';
import { useDebouncedStore, useStore } from './stores';
import { saveAll } from 'tauri-plugin-pinia/src/index.ts';

const store = useStore();
const { start, stop } = store.$tauri;

const debouncedStore = useDebouncedStore();
const { start: startDebounced, stop: stopDebounced } = debouncedStore.$tauri;

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

onMounted(() => {
  void start();
  void startDebounced();
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
