<script setup lang="ts">
import type { VNode } from 'vue';
import ExternalLink from './ExternalLink.vue';
import type { Plugin } from '@/data/plugin.data';

interface Props {
  path: string;
  plugin?: Plugin;
}

defineProps<Props>();

defineSlots<{ default: () => VNode }>();
</script>

<template>
  <ExternalLink :href="path" kind="reference" :plugin>
    <slot></slot>
  </ExternalLink>
</template>
