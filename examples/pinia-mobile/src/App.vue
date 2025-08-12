<script setup lang="ts">
import { useStore } from './store';
import { clearAutosave, save, saveNow, setAutosave } from '@tauri-store/pinia/src/index.js';

const store = useStore();
const { start, stop } = store.$tauri;
</script>

<template>
  <main>
    <div class="action">
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
      </div>
    </section>
  </main>
</template>
