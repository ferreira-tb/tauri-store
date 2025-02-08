<script lang="ts">
  import { Link } from '$components/link';
  import * as Sidebar from '$components/base/sidebar';
  import { useSidebar } from '$components/base/sidebar';

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
