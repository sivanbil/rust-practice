use std::fs::DirBuilder;

fn main() {
    let dir_structure = "/tmp/dir1/dir2/dir3";
    let dir = DirBuilder::new()
        .recursive(true)
        .create(dir_structure)
        .unwrap();
    println!("dir: {:?}", dir);
}