use rp::item_storage;

#[test]
fn can_use_item_storage_module() {
    assert_eq!(1234, item_storage::call_me());
}