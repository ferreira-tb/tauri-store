<script lang="ts">
  import { plugins } from '$lib/data';
  import { Button } from '../ui/button';
  import { resolvePluginIcon } from '$lib/icon';
  import * as Dialog from '$lib/components/ui/dialog';
  import { currentPlugin, DEFAULT_PLUGIN } from '$lib/stores/plugin';

  let isOpen = $state(false);

  const CurrentIcon = $derived.by(() => {
    return resolvePluginIcon($currentPlugin ?? DEFAULT_PLUGIN);
  });

  function onClick(plugin: string) {
    currentPlugin.set(plugin);
    isOpen &&= false;
  }
</script>

<Dialog.Root bind:open={isOpen}>
  <Dialog.Trigger>
    {#snippet child({ props })}
      <button {...props} type="button" class="text-muted-foreground">
        <CurrentIcon size="1.5rem" />
      </button>
    {/snippet}
  </Dialog.Trigger>
  <Dialog.Portal>
    <Dialog.Content class="rounded-lg">
      <Dialog.Title class="w-full text-center">
        <span>Select a plugin</span>
      </Dialog.Title>

      <div class="flex max-h-[70vh] flex-wrap justify-center gap-2 py-2">
        {#each plugins as plugin (plugin.name)}
          {@const Icon = resolvePluginIcon(plugin.name as TauriPlugin)}
          <Button
            variant="ghost"
            onclick={() => onClick(plugin.name)}
            class="flex items-center gap-2 p-4"
          >
            <Icon />
            <span>{plugin.name}</span>
          </Button>
        {/each}
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
