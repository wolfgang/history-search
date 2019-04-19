use std::path::Path;
use std::fs::{remove_dir_all, File, OpenOptions};
use std::io::prelude::*;
use regex::Regex;

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

#[test]
fn read_items_returns_empty_vector_if_home_dir_is_fresh() {
    remove_home_dir();
    let item_storage = ItemStorage::new(HOME_DIR);
    assert_eq!(0, item_storage.read_items().len());
}

#[test]
fn read_items_returns_items_sorted_by_timestamp_descending() {
    remove_home_dir();
    let item_storage = ItemStorage::new(HOME_DIR);
    write_items_file("1 entry1\n2 entry2\n3 entry3");
    assert_eq!(vec!("entry3", "entry2", "entry1"), item_storage.read_items());
}

#[test]
fn add_item_adds_item_with_timestamp_to_file() {
    remove_home_dir();
    let item_storage = ItemStorage::new(HOME_DIR);
    write_items_file("1 entry1\n");

    let mut args = vec!(String::from("second"), String::from("entry"));
    item_storage.add_item(&mut args).unwrap();
    assert_items_file_matches(r"1 entry1\n\d+ second entry");
}

#[test]
fn add_item_does_not_add_duplicates() {
    remove_home_dir();
    let item_storage = ItemStorage::new(HOME_DIR);
    write_items_file("1 first entry\n");
    let mut args = vec!(String::from("first"), String::from("entry"));
    item_storage.add_item(&mut args).unwrap();
    assert_items_file_matches(r"^1 first entry\n$");
}

#[test]
fn add_item_add_current_dir_if_minus_d_option_is_given() {
    remove_home_dir();
    let item_storage = ItemStorage::new(HOME_DIR);
    let mut args = vec!(String::from("-d"), String::from("entry"));
    item_storage.add_item(&mut args).unwrap();    
    assert_items_file_matches(r"^\d+ \[.*/replay\]entry\n$");
}

fn remove_home_dir()  {
    let home_dir = format!("/tmp/replay_test");
    remove_dir_all(&home_dir).unwrap_or_default();    
}

fn write_items_file(contents: &str) {
    let mut file = OpenOptions::new()
                .write(true)
                .open(ITEMS_FILE)
                .expect("Failed to open items file");            
    file.write_all(contents.as_bytes()).unwrap();
}

fn assert_items_file_matches(regex_str: &str) {
    let mut file = File::open(ITEMS_FILE).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(regex_str).unwrap();
    assert!(re.is_match(&contents), format!("Item file contents don't match: \n{}", contents));

}

fn assert_path_exists(path: &str) {
    assert!(Path::new(path).exists(), format!("Expected path {} to exist", path));
}