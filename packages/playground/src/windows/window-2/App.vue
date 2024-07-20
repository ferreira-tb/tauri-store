<script setup lang="ts">
import { useDebouncedStore, useStore } from '../../stores';

const store = useStore();
void store.$tauri.start();

const debouncedStore = useDebouncedStore();
void debouncedStore.$tauri.start();
</script>

<template>
  <main>
    <h1>Window 2</h1>
    <section>
      <p>Counter: {{ store.counter }}</p>
      <button type="button" @click="store.increment">Increment</button>
    </section>

    <section>
      <p>Debounced Counter: {{ debouncedStore.debouncedCounter }}</p>
      <button type="button" @click="debouncedStore.increment">Increment Debounced</button>
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
  margin-bottom: 1rem;
}
</style>
