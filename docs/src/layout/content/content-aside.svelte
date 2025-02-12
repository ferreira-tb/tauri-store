<script lang="ts">
  import { onDestroy } from 'svelte';
  import { Link } from '$components/link';
  import type { Headings } from './aside.svelte';
  import { currentPlugin } from '$stores/plugin';

  type Props = {
    headings: Headings;
  };

  const { headings }: Props = $props();

  let isFirstRun = true;
  const unsubscribe = currentPlugin.subscribe(() => {
    if (isFirstRun) {
      isFirstRun = false;
      return;
    }

    headings.update();
  });

  onDestroy(() => {
    unsubscribe();
  });
</script>

<aside class="relative hidden w-full max-w-64 xl:block">
  {#if headings.value.length > 0}
    {#key $currentPlugin}
      <div class="sticky top-16 flex flex-col gap-2 pt-8">
        <span class="text-lg font-semibold">On this page</span>
        <nav class="flex flex-col gap-1">
          {#each headings.value as heading (heading.link)}
            <Link href={heading.link} title={heading.title}>{heading.title}</Link>
          {/each}
        </nav>
      </div>
    {/key}
  {/if}
</aside>
