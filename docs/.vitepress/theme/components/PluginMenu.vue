<script setup lang="ts">
import Flyout from './Flyout.vue';
import { useCurrentPlugin } from '../composables/plugin';
import VPMenuLink from 'vitepress/dist/client/theme-default/components/VPMenuLink.vue';

const { plugin, icon } = useCurrentPlugin();

const items: { link: string; text: PluginName }[] = [
  { text: 'tauri-store', link: '/guide/getting-started' },
  { text: '@tauri-store/pinia', link: '/plugin-pinia/guide/getting-started' },
  { text: '@tauri-store/svelte', link: '/plugin-svelte/guide/getting-started' },
  { text: '@tauri-store/valtio', link: '/plugin-valtio/guide/getting-started' },
  { text: '@tauri-store/zustand', link: '/plugin-zustand/guide/getting-started' },
];
</script>

<template>
  <Flyout label="Select a plugin">
    <VPMenuLink v-for="item of items" :key="item.text" :item />

    <template #trigger>
      <div class="trigger">
        <div class="icon">
          <component :is="icon" size="2rem" />
        </div>
        <div class="label">
          <span>Select a plugin</span>
          <span>{{ plugin }}</span>
        </div>
      </div>
    </template>
  </Flyout>
</template>

<style scoped>
.trigger {
  display: flex;
  justify-content: flex-start;
  align-items: center;
  width: 100%;
}

.icon {
  display: flex;
  justify-content: center;
  align-items: center;
  margin-right: 0.75rem;
  width: 2rem;
  height: 2rem;
}

.label {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  transition: color 0.25s;
  color: var(--vp-c-text-1);
  line-height: 1.5;
}

.label span:first-child {
  color: var(--vp-c-text-2);
  font-size: 1em;
}

.label span:last-child {
  font-weight: 500;
  font-size: 1.2em;
}
</style>
