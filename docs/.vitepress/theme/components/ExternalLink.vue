<script setup lang="ts">
import { computed, type VNode } from 'vue';
import type { Plugin } from '@/data/plugin.data';
import { docsRs, reference } from '../../shared/url';
import { useSelectedPlugin } from '@/composables/plugin';

interface Props {
  href: string;
  kind?: 'docs.rs' | 'reference' | 'custom';
  plugin?: Plugin;
}

const props = withDefaults(defineProps<Props>(), {
  kind: 'custom',
});

defineSlots<{ default: () => VNode }>();

const selectedPlugin = useSelectedPlugin();
const targetPlugin = computed(() => props.plugin ?? selectedPlugin.value);

const url = computed(() => {
  switch (props.kind) {
    case 'docs.rs':
      return docsRs(targetPlugin.value, props.href);
    case 'reference':
      return reference(targetPlugin.value, props.href);
    case 'custom':
    default:
      return props.href;
  }
});
</script>

<template>
  <a :href="url" target="_blank" rel="noopener noreferrer">
    <slot></slot>
  </a>
</template>
