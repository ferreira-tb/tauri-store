<script lang="ts" setup>
import { ref, type VNode } from 'vue';
import VPMenu from 'vitepress/dist/client/theme-default/components/VPMenu.vue';

defineProps<{
  label?: string;
}>();

defineSlots<{
  default: () => VNode;
  trigger: () => VNode;
}>();

const open = ref(false);
</script>

<template>
  <div class="flyout" @mouseenter="open = true" @mouseleave="open = false">
    <button
      type="button"
      class="button"
      aria-haspopup="true"
      :aria-expanded="open"
      :aria-label="label"
      @click="open = !open"
    >
      <slot name="trigger"></slot>
    </button>

    <div class="menu">
      <VPMenu>
        <slot></slot>
      </VPMenu>
    </div>
  </div>
</template>

<style scoped>
.flyout {
  position: relative;
}

.flyout:hover {
  transition: color 0.25s;
  color: var(--vp-c-brand-1);
}

.button[aria-expanded='false'] + .menu {
  transform: translateY(0);
  visibility: hidden;
  opacity: 0;
}

.flyout:hover .menu,
.button[aria-expanded='true'] + .menu {
  transform: translateY(0);
  visibility: visible;
  opacity: 1;
}

.button {
  display: flex;
  align-items: center;
  transition: color 0.5s;
  padding: 0;
  width: 100%;
  height: var(--vp-nav-height);
  color: var(--vp-c-text-1);
}

.menu {
  position: absolute;
  top: calc(var(--vp-nav-height) / 2 + 25px);
  right: 0;
  left: 0;
  visibility: hidden;
  opacity: 0;
  z-index: 10;
  transition:
    opacity 0.25s,
    visibility 0.25s,
    transform 0.25s;
}
</style>
