<script setup lang="ts">
import DefaultTheme from 'vitepress/theme';
import PluginSelect from '@/components/PluginSelect.vue';

// eslint-disable-next-line @typescript-eslint/naming-convention
const { Layout: DefaultLayout } = DefaultTheme;
</script>

<template>
  <DefaultLayout>
    <template #sidebar-nav-before>
      <PluginSelect />
    </template>
  </DefaultLayout>
</template>
