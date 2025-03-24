<script setup lang="ts">
import '../assets/style.css';
import { onMounted } from 'vue';
import { onKeyDown } from '@vueuse/core';
import { exit } from '@tauri-apps/plugin-process';
import { openStore, useStore } from './utils/store';
import { onError, printStore } from './utils/commands';
import { clearAutosave, save, saveNow, setAutosave } from '@tauri-store/pinia/src/index.js';

const store = useStore();
const { start, stop } = store.$tauri;

onKeyDown('Escape', () => void exit(0).catch(onError));

onMounted(() => {
  start().catch(onError);
});
</script>

<template>
  <main>
    <div class="action">
      <button type="button" @click="() => setAutosave(5000)">Set Autosave</button>
      <button type="button" @click="clearAutosave">Clear Autosave</button>
    </div>
    <section id="counter">
      <p>Counter: {{ store.counter }} Nested: {{ store.nested.foo.bar.baz }}</p>
      <div class="action">
        <button type="button" @click="store.increment">Increment</button>
        <button type="button" @click="store.incrementNested">Increment Nested</button>
        <button type="button" @click="start">Start</button>
        <button type="button" @click="stop">Stop</button>
        <button type="button" @click="save(store.$id)">Save</button>
        <button type="button" @click="saveNow(store.$id)">Save Now</button>
        <button type="button" @click="printStore">Print</button>
        <button type="button" @click="openStore">Open</button>
      </div>
    </section>
  </main>
</template>
