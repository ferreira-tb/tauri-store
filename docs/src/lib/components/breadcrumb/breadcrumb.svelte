<script lang="ts">
  import * as Breadcrumb from '../base/breadcrumb';

  interface Props {
    current: string;
    parents: string[];
  }

  const { current, parents }: Props = $props();
</script>

<Breadcrumb.Root>
  <Breadcrumb.List class="list-none">
    {#each parents as parent (parent)}
      <Breadcrumb.Item>{parent}</Breadcrumb.Item>
      <Breadcrumb.Separator />
    {/each}
    <Breadcrumb.Item>
      <Breadcrumb.Page class="font-semibold">
        {current}
      </Breadcrumb.Page>
    </Breadcrumb.Item>
  </Breadcrumb.List>
</Breadcrumb.Root>
