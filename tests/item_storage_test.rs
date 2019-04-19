use std::path::Path;
use std::fs::{remove_dir_all};
use uuid::Uuid;

use rp::item_storage::ItemStorage;

#[test]
fn new_creates_home_dir() {
    let home_dir = &setup();
    assert!(!Path::new(home_dir).exists(), "Home directory already exists");
    let _item_storage = ItemStorage::new(home_dir);
    assert!(Path::new(home_dir).exists(), "Home directory was not created");
}

#[test]
fn new_does_not_create_home_dir_if_it_already_exists() {
    let home_dir = &setup();
    let _item_storage1 = ItemStorage::new(home_dir);
    let _item_storage2 = ItemStorage::new(home_dir);
}

fn setup() -> String {
    let home_dir = format!("/tmp/replay_test_{}", Uuid::new_v4().to_string());
    remove_dir_all(&home_dir).unwrap_or_default();    
    home_dir
}