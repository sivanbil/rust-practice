# integ-test-example crate document
This is a project to test `rustdoc`
[Here is a link!](https://www.rust-lang.org)

// 函数名
pub fn get_process_id() -> u32 {}

// 示例
```rust
use integ_test_example;
fn get_id() -> i32 {
    let my_pid = get_process_id()
    println!("Process id for current process is:{}", my_pid);
}
```