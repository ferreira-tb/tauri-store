<script setup lang="ts">
import { useStore } from '../../stores';

const store = useStore();
</script>

<template>
  <main>
    <h1>Window 1</h1>
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
