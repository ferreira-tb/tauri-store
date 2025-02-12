<script lang="ts">
  import { Link } from '$components/link';
  import * as DropdownMenu from '$components/base/dropdown-menu';

  type Props = {
    external?: boolean;
    items: MenuItem[];
    label: string;
  };

  type MenuItem = {
    label: string;
    url: string;
  };

  const { items, label, external }: Props = $props();
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger>
    {#snippet child({ props })}
      <span {...props} class="cursor-pointer">{label}</span>
    {/snippet}
  </DropdownMenu.Trigger>
  <DropdownMenu.Content align="end" preventScroll={false}>
    {#each items as item (item.label)}
      <DropdownMenu.Item>
        <Link href={item.url} {external} class="w-full">
          <button type="button" class="flex w-full items-center justify-start px-2">
            {item.label}
          </button>
        </Link>
      </DropdownMenu.Item>
    {/each}
  </DropdownMenu.Content>
</DropdownMenu.Root>
