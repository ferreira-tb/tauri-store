<script lang="ts">
  import { default as Base, type BaseProps } from './base.svelte';

  const _props: BaseProps = $props();
</script>

<Base {..._props} kind="Dependencies" />
