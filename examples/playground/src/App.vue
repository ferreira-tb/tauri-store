<script setup lang="ts">
import '../../assets/style.css';
import { vueStore } from './vue';
import { usePiniaStore } from './pinia';
import { onKeyDown } from '@vueuse/core';
import { exit } from '@tauri-apps/plugin-process';

const piniaStore = usePiniaStore();

onKeyDown('Escape', () => void exit(0));
</script>

<template>
  <main>
    <section id="counter">
      <p>Pinia: {{ piniaStore.counter }}</p>
      <p>Vue: {{ vueStore.counter }}</p>
    </section>
  </main>
</template>
