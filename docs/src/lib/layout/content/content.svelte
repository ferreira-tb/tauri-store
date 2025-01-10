<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';
  import type { Headings } from './aside.svelte';
  import ContentAside from './content-aside.svelte';
  import ContentFooter from './content-footer.svelte';
  import { Separator } from '$lib/components/ui/separator';

  interface Props {
    children: Snippet;
    headings: Headings;
    navbarHeight: string;
  }

  const { navbarHeight, children, headings }: Props = $props();
</script>

<main
  style:padding-top={navbarHeight}
  class={cn('z-0 m-0 flex min-h-screen w-full flex-shrink-0 flex-grow justify-center')}
>
  <div
    class="mx-auto flex w-full min-w-0 flex-col justify-between px-[--content-padding] md:px-8 lg:px-20 xl:px-24 2xl:px-36"
  >
    <div>
      {@render children()}
    </div>

    <div>
      <Separator class="mt-24" />
      <ContentFooter />
    </div>
  </div>

  <ContentAside {headings} />
</main>
