<script lang="ts">
  import { Link } from '$lib/components/link';
  import * as Sidebar from '$lib/components/ui/sidebar';
  import { useSidebar } from '$lib/components/ui/sidebar';

  interface Props {
    external?: boolean;
    href: string;
    label: string;
  }

  const { href, label, external }: Props = $props();

  const sidebar = useSidebar();

  function closeMobileSidebar() {
    if (sidebar.isMobile) {
      sidebar.openMobile &&= false;
    }
  }
</script>

<Sidebar.MenuItem>
  <Sidebar.MenuButton>
    {#snippet child({ props })}
      <Link {...props} {href} {external} onclick={closeMobileSidebar}>
        {label}
      </Link>
    {/snippet}
  </Sidebar.MenuButton>
</Sidebar.MenuItem>
