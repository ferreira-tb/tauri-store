<script lang="ts">
  import { Ext } from '$lib/components/link';
  import * as Alert from '$lib/components/alert';
  import { currentMetadata } from '$lib/stores/plugin';

  const url = $derived.by(() => {
    const docs = $currentMetadata.docs.javascript;
    return `${docs}/classes/Store.html#start`;
  });
</script>

<Alert.Root class="pb-4">
  <Alert.Title>Tip</Alert.Title>
  <Alert.Description>
    Stores won't be saved nor synchronized until you call the
    <Ext href={url} code>start</Ext> method.
  </Alert.Description>
</Alert.Root>
