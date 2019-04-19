use rp::item_storage::ItemStorage;

#[test]
fn initialize_item_storage_object() {
    let _item_storage = ItemStorage::new("/tmp/item_storage_test");
}