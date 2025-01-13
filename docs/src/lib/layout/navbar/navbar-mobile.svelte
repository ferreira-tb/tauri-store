<script lang="ts">
  import { tick } from 'svelte';
  import { Link } from '$components/link';
  import { useSidebar } from '$layout/sidebar';
  import * as Sheet from '$components/ui/sheet';
  import { Menu, PanelRightOpen } from 'lucide-svelte';
  import { Separator } from '$components/ui/separator';
  import type { Headings } from '$layout/content/aside.svelte';
  import { NAVBAR_HEIGHT, NAVBAR_HEIGHT_MOBILE } from './constants';

  interface Props {
    headings: Headings;
  }

  const { headings }: Props = $props();

  const sidebar = useSidebar();
  let isSheetOpen = $state(false);

  async function closeSheet() {
    await tick();
    isSheetOpen &&= false;
  }

  async function openSidebar() {
    await tick();
    sidebar.toggle();
  }
</script>

<div class="flex w-full justify-center">
  <Separator class="w-[calc(100%-2rem)]" />
</div>

<div
  style:height={`calc(${NAVBAR_HEIGHT_MOBILE} - ${NAVBAR_HEIGHT})`}
  class="flex items-center justify-between px-4"
>
  <button type="button" onclick={openSidebar} class="navbar-button-mobile">
    <Menu class="mr-[8px]" />
    <span class="w-full text-start">Menu</span>
  </button>

  {#if headings.value.length > 0}
    <Sheet.Root bind:open={isSheetOpen}>
      <Sheet.Trigger class="navbar-button-mobile">
        <span class="w-full text-end">On this page</span>
        <PanelRightOpen class="ml-[8px] size-5" />
      </Sheet.Trigger>
      <Sheet.Content>
        <Sheet.Header>
          <Sheet.Title class="mb-4 flex w-full justify-start">
            <span>On this page</span>
          </Sheet.Title>
        </Sheet.Header>

        <div class="flex flex-col gap-1">
          {#each headings.value as heading (heading.link)}
            <Link href={heading.link} title={heading.title}>
              <button
                type="button"
                onclick={closeSheet}
                class="text-muted-foreground flex w-full items-center justify-start px-2"
              >
                {heading.title}
              </button>
            </Link>
          {/each}
        </div>
      </Sheet.Content>
    </Sheet.Root>
  {/if}
</div>
