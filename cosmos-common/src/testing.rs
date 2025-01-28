use std::{
    fs,
    path::{Path, PathBuf},
};

use base64::{prelude::BASE64_STANDARD, Engine};
use prost::Message;
use crate::pb::sf::cosmos::r#type::v2::Block;

pub fn read_block(filename: &str) -> Block {
    let encoded = fs::read_to_string(testdata_file(filename)).expect("Failed to read file");
    let raw_bytes = BASE64_STANDARD
        .decode(&encoded)
        .expect("Failed to decode base64");

    Block::decode(&*raw_bytes).expect("Not able to decode Block")
}

fn testdata_file(test_filename: &str) -> PathBuf {
    Path::parent(Path::new(file!()))
        .expect("Failed to get current source file directory")
        .join(test_filename)
}