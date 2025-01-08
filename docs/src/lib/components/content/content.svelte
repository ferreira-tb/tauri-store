<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';
  import type { Headings } from './aside.svelte';
  import ContentAside from './content-aside.svelte';

  interface Props {
    children: Snippet;
    headings: Headings;
    navbarHeight: string;
  }

  const { navbarHeight, children, headings }: Props = $props();
</script>

<main
  style:padding-top={navbarHeight}
  class={cn('z-0 m-0 flex w-full flex-shrink-0 flex-grow justify-center pb-32')}
>
  <div
    class="mx-auto w-full min-w-0 px-[--content-padding] pt-8 md:px-8 lg:px-20 xl:px-24 2xl:px-36"
  >
    {@render children()}
  </div>

  <ContentAside {headings} />
</main>
