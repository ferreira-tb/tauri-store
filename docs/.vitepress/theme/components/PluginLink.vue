<script setup lang="ts">
import { computed, type VNode } from 'vue';
import { useSelectedPlugin } from '@/composables/plugin';
import { type Plugin, data as plugins } from '@/data/plugin.data';

interface Props {
  path: string;
  plugin?: Plugin | string;
}

const props = defineProps<Props>();

defineSlots<{ default: () => VNode }>();

const selectedPlugin = useSelectedPlugin();
const targetPlugin = computed(() => {
  const selected = selectedPlugin.value;
  if (typeof props.plugin === 'string') {
    return plugins.find(({ name }) => name === props.plugin) ?? selected;
  } else if (props.plugin) {
    return props.plugin;
  }

  return selected;
});

const url = computed(() => {
  return `${props.path}?plugin=${targetPlugin.value.name}`;
});
</script>

<template>
  <a :href="url">
    <slot></slot>
  </a>
</template>
