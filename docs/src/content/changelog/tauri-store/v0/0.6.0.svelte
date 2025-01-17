<script lang="ts">
  import { Ext } from '$components/link';
  import { Changelog } from '$components/container';

  const version = '0.6.0';

  // prettier-ignore
  const url = {
    "ManagerExt::with_store": "https://docs.rs/tauri-store/0.6.0/tauri_store/trait.ManagerExt.html#method.with_store",
    "Store::patch": "https://docs.rs/tauri-store/0.6.0/tauri_store/struct.Store.html#method.patch",
    "Store::patch_with_source": "https://docs.rs/tauri-store/0.6.0/tauri_store/struct.Store.html#method.patch_with_source",
    "Store::set": "https://docs.rs/tauri-store/0.6.0/tauri_store/struct.Store.html#method.set",
    "Store::try_get_or": "https://docs.rs/tauri-store/0.6.0/tauri_store/struct.Store.html#method.try_get_or",
    "Store::try_get_or_default": "https://docs.rs/tauri-store/0.6.0/tauri_store/struct.Store.html#method.try_get_or_default",
    "Store::try_get_or_else": "https://docs.rs/tauri-store/0.6.0/tauri_store/struct.Store.html#method.try_get_or_else",
    "StoreCollection::patch": "https://docs.rs/tauri-store/0.6.0/tauri_store/struct.StoreCollection.html#method.patch",
    "StoreCollection::set": "https://docs.rs/tauri-store/0.6.0/tauri_store/struct.StoreCollection.html#method.set",
    "StoreCollection::state": "https://docs.rs/tauri-store/0.6.0/tauri_store/struct.StoreCollection.html#method.state",
    "StoreCollection::try_state": "https://docs.rs/tauri-store/0.6.0/tauri_store/struct.StoreCollection.html#method.try_state",
    "StoreCollection::try_get_or": "https://docs.rs/tauri-store/0.6.0/tauri_store/struct.StoreCollection.html#method.try_get_or",
    "StoreCollection::try_get_or_default": "https://docs.rs/tauri-store/0.6.0/tauri_store/struct.StoreCollection.html#method.try_get_or_default",
    "StoreCollection::try_get_or_else": "https://docs.rs/tauri-store/0.6.0/tauri_store/struct.StoreCollection.html#method.try_get_or_else"
  }
</script>

{#snippet ext(key: keyof typeof url)}
  <Ext href={url[key]} code>{key}</Ext>
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
