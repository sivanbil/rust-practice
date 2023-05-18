use std::fs;
use std::fs::File;
use std::fs::OpenOptions;

fn main() {
    // create
    let f = File::create("hello-rust.txt").expect("Unable to create file");

    // open
    let file2 = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("hello-rust-2.txt")
        .expect("Unable to open file");
    // read
    let content = fs::read_to_string("hello.txt").expect("failed to read file");
    println!("The content of the file is: {}", content);

    fs::copy("hello.txt", "hello2.txt").expect("Unable to copy file");

    fs::write("hello3.txt", "hello world").expect("Unable to write file");

    let contentBytes = fs::read("hello3.txt").expect("failed to read file");
    println!("The content of the file is: {:?}", String::from_utf8(contentBytes).unwrap());

    // copy
    // fs::copy("hello.txt", "hello2.txt").expect("Unable to copy file");
    
    // // rename
    // fs::rename("hello.txt", "hello3.txt").expect("Unable to rename file");


}