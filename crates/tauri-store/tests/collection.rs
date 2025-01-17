#![allow(unused_must_use)]

mod shared;

use shared::{assert_exists, temp_dir, with_collection, StoreExt, STORE_ID};

#[tokio::test]
async fn set_path() {
  with_collection(|collection| {
    let store_path = collection
      .with_store(STORE_ID, |store| {
        store.set("key", 42).unwrap();
        store.save_now().unwrap();
        store.assert_exists(true);
        store.path()
      })
      .unwrap();

    let do_not_move = collection.path().join("do-not-move");
    std::fs::write(&do_not_move, "LEAVE ME ALONE").unwrap();

    let new_path = temp_dir().join("test-set-path");
    collection.set_path(&new_path);

    assert_exists(&store_path, false);
    assert_exists(&new_path, true);
    assert_exists(&do_not_move, true);

    let new_store_path = collection
      .with_store(STORE_ID, |store| store.path())
      .unwrap();

    assert_exists(&new_store_path, true);
    assert_ne!(store_path, new_store_path);
  })
  .await;
}
