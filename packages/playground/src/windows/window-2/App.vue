<script setup lang="ts">
import { useStore } from '../../stores';

const store = useStore();
void store.$tauri.start();
</script>

<template>
  <main>
    <h1>Window 2</h1>
    <p>Counter: {{ store.counter }}</p>
    <button type="button" @click="store.increment">Increment</button>
    <button type="button" @click="store.decrement">Decrement</button>
  </main>
</template>

<style scoped>
main {
  display: flex;
  flex-direction: column;
}
</style>
