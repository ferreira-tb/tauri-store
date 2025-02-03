<script lang="ts">
  import { HOME } from '$lib/url';
  import { Link } from '$components/link';
  import { Button } from '$components/ui/button';
  import { Container } from '$components/container';
</script>

<svelte:head>
  <title>Not found | tauri-store</title>
  <meta name="description" content="Persistent stores for Tauri" />
</svelte:head>

<div>
  <Container title="Page not found" level={1}>
    <Link href={HOME}>
      <Button>Take me home</Button>
    </Link>
  </Container>
</div>
