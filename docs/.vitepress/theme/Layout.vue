<script setup lang="ts">
import DefaultTheme from 'vitepress/theme';
import PluginMenu from './components/PluginMenu.vue';

const { Layout: DefaultLayout } = DefaultTheme;
</script>

<template>
  <DefaultLayout>
    <template #sidebar-nav-before>
      <div class="sidebar-nav-before">
        <PluginMenu />
      </div>
    </template>
  </DefaultLayout>
</template>

<style scoped>
.sidebar-nav-before {
  padding-top: 1rem;
}
</style>
