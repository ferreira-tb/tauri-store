<script lang="ts">
  import { Link } from '$lib/components/link';
  import { Button } from '$lib/components/ui/button';
  import { Container } from '$lib/components/container';
</script>

<svelte:head>
  <title>Not found | tauri-store</title>
  <meta name="description" content="Persistent stores for Tauri" />
</svelte:head>

<div>
  <Container title="Page not found" level={1}>
    <Link href="https://tb.dev.br/tauri-store/">
      <Button>Take me home</Button>
    </Link>
  </Container>
</div>
