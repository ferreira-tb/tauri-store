<script setup lang="ts">
import type { Plugin } from '../../../types';
import { inject, symbols } from '@/utils/symbols';
import { data as plugins } from '@/data/plugin.data.js';
import { useSelectedPlugin } from '@/composables/plugin';
import { computedWithControl, watchDeep } from '@vueuse/core';
import VPFlyout from 'vitepress/dist/client/theme-default/components/VPFlyout.vue';
import VPMenuLink from 'vitepress/dist/client/theme-default/components/VPMenuLink.vue';
import { computed, nextTick, onMounted, ref, watchEffect, watchSyncEffect } from 'vue';

const mounted = ref(false);
const selectedPlugin = useSelectedPlugin();
const route = inject(symbols.currentRoute);
const style = ref(createStyle());

const pluginParam = computedWithControl(route, () => {
  if (mounted.value) {
    try {
      const url = new URL(location.href);
      return url.searchParams.get('plugin');
    } catch {
      return null;
    }
  }

  return null;
});

const isParamMismatch = computed(() => {
  return (
    pluginParam.value &&
    typeof pluginParam.value === 'string' &&
    selectedPlugin.value.name !== pluginParam.value
  );
});

watchEffect(() => {
  if (mounted.value && isParamMismatch.value) {
    setPlugin(plugins.find(({ name }) => name === pluginParam.value));
  }
});

watchDeep([selectedPlugin, mounted], () => {
  if (mounted.value) {
    const classList = document.documentElement.classList;
    for (const { name } of plugins) {
      classList.remove(toClassName(name));
    }

    classList.add(toClassName(selectedPlugin.value.name));
  }
});

watchSyncEffect(() => {
  if (mounted.value) {
    const id = 'tauri-plugin-preference';
    let styleEl = document.querySelector(`style#${id}`);

    if (!styleEl) {
      styleEl = document.createElement('style');
      styleEl.setAttribute('id', id);
      styleEl.setAttribute('type', 'text/css');
      document.head.append(styleEl);
    }

    styleEl.textContent = style.value;
  }
});

function setPlugin(plugin?: Plugin) {
  if (plugin) selectedPlugin.value = plugin;
}

function createStyle() {
  const none = plugins.map(({ name }) => {
    return `.${name} { display: none; }`;
  });

  const prefer = plugins.map(({ name }) => {
    return `.${toClassName(name)} .${name} { display: initial; }`;
  });

  return none.concat(prefer).join('\n');
}

function toClassName(name: string) {
  return `prefer-${name}`;
}

function getMenuLinkItem(plugin: Plugin) {
  return {
    text: plugin.name,
    link: `?plugin=${plugin.name}`,
  };
}

async function triggerPluginParam() {
  await nextTick();
  pluginParam.trigger();
}

onMounted(() => {
  mounted.value = true;
});
</script>

<template>
  <VPFlyout
    v-if="route && !route.includes('changelog')"
    label="Select a plugin"
    :button="selectedPlugin.name"
    class="ts-flyout"
  >
    <div class="items">
      <template v-for="plugin of plugins" :key="plugin.name">
        <VPMenuLink
          :item="getMenuLinkItem(plugin)"
          :class="plugin.name === selectedPlugin.name && 'ts-selected'"
          @click="triggerPluginParam"
        />
      </template>
    </div>
  </VPFlyout>
  <span v-else></span>
</template>

<style>
.ts-flyout .button {
  padding: 0;
}

.ts-flyout .menu {
  position: absolute;
  right: unset;
  z-index: 1;
}

.ts-flyout .ts-selected .link {
  color: var(--vp-c-brand-1);
}
</style>
