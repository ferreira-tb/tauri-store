<script setup lang="ts">
import { printStore } from './commands';
import { onKeyDown } from '@vueuse/core';
import { openStore, store } from './store';
import { exit } from '@tauri-apps/plugin-process';
import { clearAutosave, setAutosave } from '@tauri-store/vue/src/index.js';

const { start, stop, save, saveNow } = store.$tauri;

onKeyDown('Escape', () => void exit(0));
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
        <button type="button" @click="() => store.counter++">Increment</button>
        <button type="button" @click="() => store.nested.foo.bar.baz++">Increment Nested</button>
        <button type="button" @click="start">Start</button>
        <button type="button" @click="stop">Stop</button>
        <button type="button" @click="save">Save</button>
        <button type="button" @click="saveNow">Save Now</button>
        <button type="button" @click="printStore">Print</button>
        <button type="button" @click="openStore">Open</button>
      </div>
    </section>
  </main>
</template>
