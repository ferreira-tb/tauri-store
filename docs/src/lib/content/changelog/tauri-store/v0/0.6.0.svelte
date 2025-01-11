<script lang="ts">
  import data from './data/0.6.0.json';
  import { Ext } from '$lib/components/link';
  import { Changelog } from '$lib/components/container';

  const version = '0.6.0';
</script>

{#snippet ext(key: keyof typeof data.url)}
  <Ext href={data.url[key]} code>{key}</Ext>
{/snippet}

<Changelog.Version {version} />

<Changelog.BreakingChanges {version}>
  <ul>
    <li>
      <code>StoreState</code> is now a struct instead of a type alias.
    </li>
    <li>
      Remove the <code>StoreStateExt</code> trait.
    </li>
    <li>
      Rename <code>StoreCollection::store_state</code> to
      {@render ext('StoreCollection::state')}.
    </li>
    <li>
      Rename <code>StoreCollection::try_store_state</code> to
      {@render ext('StoreCollection::try_state')}.
    </li>
  </ul>
</Changelog.BreakingChanges>

<Changelog.Features {version}>
  <ul>
    <li>
      Add {@render ext('Store::try_get_or')},
      {@render ext('Store::try_get_or_default')}, and
      {@render ext('Store::try_get_or_else')}.
    </li>
    <li>
      Add {@render ext('StoreCollection::try_get_or')},
      {@render ext('StoreCollection::try_get_or_default')}, and
      {@render ext('StoreCollection::try_get_or_else')}.
    </li>
    <li>
      Add {@render ext('ManagerExt::with_store')} as a provided method.
    </li>
    <li>
      {@render ext('Store::patch')}, {@render ext('Store::patch_with_source')},
      {@render ext('Store::set')}, {@render ext('StoreCollection::patch')}, and
      {@render ext('StoreCollection::set')} now take a generic parameter for the value type.
    </li>
  </ul>
</Changelog.Features>
