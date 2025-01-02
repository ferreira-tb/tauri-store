<script lang="ts">
  import Link from '../link.svelte';
  import type { Headings } from './aside.svelte';

  interface Props {
    headings: Headings;
  }

  const p: Props = $props();
</script>

<aside class="relative hidden w-full max-w-64 xl:block">
  {#if p.headings.value.length > 0}
    <div class="sticky top-16 flex flex-col gap-2 pt-8">
      <span class="text-lg font-semibold">On this page</span>
      <nav class="flex flex-col gap-1">
        {#each p.headings.value as heading (heading.link)}
          <Link href={heading.link} title={heading.title}>{heading.title}</Link>
        {/each}
      </nav>
    </div>
  {/if}
</aside>
