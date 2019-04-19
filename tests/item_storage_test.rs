use std::path::Path;
use std::fs::{remove_dir_all};

use rp::item_storage::ItemStorage;

const HOME_DIR : &str = "/tmp/replay_test";

#[test]
fn new_creates_home_dir() {
    setup();
    assert_path_exists(Path::new(HOME_DIR));
    let _item_storage = ItemStorage::new(HOME_DIR);
    assert!(Path::new(HOME_DIR).exists(), "Home directory was not created");
}

#[test]
fn new_does_not_create_home_dir_if_it_already_exists() {
    let _item_storage1 = ItemStorage::new(HOME_DIR);
    let _item_storage2 = ItemStorage::new(HOME_DIR);
}

#[test]
fn new_creates_item_file_if_home_dir_does_not_exist() {
    setup();
    ItemStorage::new(HOME_DIR);
    assert!(Path::new(HOME_DIR).join("items.txt").exists(), "Item file was not created");

}

fn setup()  {
    let home_dir = format!("/tmp/replay_test");
    remove_dir_all(&home_dir).unwrap_or_default();    
}

fn assert_path_exists(path: &Path) {
    assert!(path.exists(), format!("Expected path {} to exist", path.to_str().unwrap()));
}