use std::fs;
use std::path::Path;

fn main() {
    let dirs = fs::read_dir(".").expect("Unable to read dir");

    for dir in dirs {
        let dir = dir.unwrap();
        let dir_path = dir.path();

        let dir_metadata = dir.metadata().expect("Unable to get metadata");

        let dir_type = dir.file_type().expect("Unable to get file type");

        let dir_name = dir.file_name().into_string().expect("Unable to get file name");

        println!("Path:{:?}, meta:{:?}, type:{:?}, Name: {:?}", dir_path, dir_metadata, dir_type, dir_name);
    }

}