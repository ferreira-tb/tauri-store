<script setup lang="ts">
import VPBadge from 'vitepress/dist/client/theme-default/components/VPBadge.vue';

interface Props {
  name: 'file-sync-all' | 'unstable-async';
}

defineProps<Props>();
</script>

<template>
  <VPBadge type="info" :text="name" />
</template>
