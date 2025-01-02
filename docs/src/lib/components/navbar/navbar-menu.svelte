<script lang="ts">
  import Link from '../link.svelte';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu';

  interface Props {
    external?: boolean;
    items: MenuItem[];
    label: string;
  }

  interface MenuItem {
    label: string;
    url: string;
  }

  const p: Props = $props();
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger>
    {#snippet child({ props })}
      <span {...props} class="cursor-pointer">{p.label}</span>
    {/snippet}
  </DropdownMenu.Trigger>
  <DropdownMenu.Content align="end" preventScroll={false}>
    {#each p.items as item (item.label)}
      <DropdownMenu.Item>
        <Link href={item.url} external={p.external}>
          <button type="button" class="flex w-full items-center justify-start px-2">
            {item.label}
          </button>
        </Link>
      </DropdownMenu.Item>
    {/each}
  </DropdownMenu.Content>
</DropdownMenu.Root>
