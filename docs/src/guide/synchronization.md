# Synchronization

Whenever the state changes, the store notifies Rust to keep the frontend and backend in sync. However, since data is serialized with each notification, frequent updates can be resource-intensive. One way to address this issue is by applying debouncing or throttling, making the synchronization process more efficient.

<div class="tauri-plugin-pinia">

<!--@include: ../examples/synchronization/pinia.md-->

</div>

::: tip
For a detailed explanation of the differences between debouncing and throttling, take a look at [this article](https://kettanaito.com/blog/debounce-vs-throttle).
:::

While this process isn’t directly related to [store persistence](./persisting-state.md), it can still impact what gets saved. When the store is saved, the data written to disk comes from Rust’s cache at that moment. If the synchronization hasn’t finished yet, Rust might still be working with outdated values.
