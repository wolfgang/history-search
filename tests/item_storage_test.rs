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
    remove_home_dir();
    ItemStorage::new(HOME_DIR);
    ItemStorage::new(HOME_DIR);
}

#[test]
fn new_creates_item_file_if_home_dir_does_not_exist() {
    remove_home_dir();
    ItemStorage::new(HOME_DIR);
    assert_path_exists(ITEMS_FILE);
}

#[test]
fn read_items_returns_empty_vector_if_home_dir_is_fresh() {
    let item_storage = fresh_item_storage();
    assert_eq!(0, item_storage.read_items().len());
}

#[test]
fn read_items_returns_items_sorted_by_timestamp_descending() {
    let item_storage = fresh_item_storage();

    write_items_file("1 entry1\n2 entry2\n3 entry3");
    assert_eq!(vec!("entry3", "entry2", "entry1"), item_storage.read_items());
}

#[test]
fn add_item_adds_item_with_timestamp_to_file() {
    let item_storage = fresh_item_storage();
    write_items_file("1 entry1\n");

    let mut args = vec!(String::from("second"), String::from("entry"));
    item_storage.add_item(&mut args).unwrap();
    assert_items_file_matches(r"1 entry1\n\d+ second entry");
}

#[test]
fn add_item_does_not_add_duplicates() {
    let item_storage = fresh_item_storage();
    write_items_file("1 first entry\n");

    let mut args = vec!(String::from("first"), String::from("entry"));
    item_storage.add_item(&mut args).unwrap();
    assert_items_file_matches(r"^1 first entry\n$");
}

#[test]
fn add_item_add_current_dir_if_minus_d_option_is_given() {
    let item_storage = fresh_item_storage();

    let mut args = vec!(String::from("-d"), String::from("entry"));
    item_storage.add_item(&mut args).unwrap();    
    assert_items_file_matches(r"^\d+ \[.*/replay\]entry\n$");
}

#[test]
#[should_panic(expected="Must add command")]
fn add_item_panics_if_no_directory_given_after_minus_d() {
    let item_storage = fresh_item_storage();

    let mut args = vec!(String::from("-d"));
    item_storage.add_item(&mut args).unwrap();        
}

#[test]
fn replace_timestamp_updates_timestamp_of_given_item() {
    let item_storage = fresh_item_storage();
    write_items_file("1 first entry\n2 second entry");
    item_storage.replace_timestamp("second entry");
    assert_items_file_matches(r"1 first entry\n\d{5,} second entry");
}


fn fresh_item_storage() -> ItemStorage {
    remove_home_dir();
    ItemStorage::new(HOME_DIR)
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