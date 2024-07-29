<script setup lang="ts">
import { onMounted } from 'vue';
import { useDebouncedStore, useStore } from '../../stores';

const store = useStore();
const { start, stop } = store.$tauri;

const debouncedStore = useDebouncedStore();
const { start: startDebounced, stop: stopDebounced } = debouncedStore.$tauri;

onMounted(() => {
  void start();
  void startDebounced();
});
</script>

<template>
  <main>
    <section id="counter">
      <p>Counter: {{ store.counter }}</p>
      <div class="action">
        <button type="button" @click="store.increment">Increment</button>
        <button type="button" @click="start">Start</button>
        <button type="button" @click="stop">Stop</button>
      </div>
    </section>

    <section id="debounced-counter">
      <p>Debounced Counter: {{ debouncedStore.debouncedCounter }}</p>
      <div class="action">
        <button type="button" @click="debouncedStore.increment">Increment</button>
        <button type="button" @click="startDebounced">Start</button>
        <button type="button" @click="stopDebounced">Stop</button>
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
