use std::path::Path;
use std::fs::{remove_dir_all};


use rp::item_storage::ItemStorage;

const HOME_DIR : &str = "/tmp/replay_test";
const ITEMS_FILE : &str = "/tmp/replay_test/items.txt";

#[test]
fn new_creates_home_dir() {
    remove_home_dir();
    assert!(!Path::new(HOME_DIR).exists(), "Home directory already exists");
    ItemStorage::new(HOME_DIR);
    assert_path_exists(HOME_DIR);
}

#[test]
fn new_does_not_create_home_dir_if_it_already_exists() {
    let _item_storage1 = ItemStorage::new(HOME_DIR);
    let _item_storage2 = ItemStorage::new(HOME_DIR);
}

#[test]
fn new_creates_item_file_if_home_dir_does_not_exist() {
    remove_home_dir();
    ItemStorage::new(HOME_DIR);
    assert_path_exists(ITEMS_FILE);
}

#[test]
fn new_does_not_create_item_file_if_it_already_exists() {
    ItemStorage::new(HOME_DIR);
    ItemStorage::new(HOME_DIR);
}

fn remove_home_dir()  {
    let home_dir = format!("/tmp/replay_test");
    remove_dir_all(&home_dir).unwrap_or_default();    
}

fn assert_path_exists(path: &str) {
    assert!(Path::new(path).exists(), format!("Expected path {} to exist", path));
}