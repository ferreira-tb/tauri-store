<script setup lang="ts">
import '../../assets/style.css';
import { onMounted } from 'vue';
import { vueStore } from './vue';
import { usePiniaStore } from './pinia';
import { onKeyDown } from '@vueuse/core';
import { exit } from '@tauri-apps/plugin-process';

const piniaStore = usePiniaStore();

onKeyDown('Escape', () => void exit(0));

onMounted(async () => {
  await piniaStore.$tauri.start();
  await vueStore.$tauri.start();
});
</script>

<template>
  <main>
    <section id="counter">
      <p>Pinia: {{ piniaStore.counter }}</p>
      <p>Vue: {{ vueStore.counter }}</p>
    </section>
  </main>
</template>
