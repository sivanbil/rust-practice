# cargo
## 介绍
> 类似java maven、gradle, php的composer的包管理器

cargo负责构建代码、下载依赖库以及构建这些库，像practice里面的hello world 因为用不到其他的
库，所以如果用cargo构建hello world,就只做了构建代码的工作。

## 使用
- 创建项目
```bash
cargo new hello_cargo
 
# Created binary (application) `hello_cargo` package
```
执行完上面的命令后目录里有一个cargo.toml（这个文件负责依赖管理）文件
```rust
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
然后目录里还有一个src子目录，里面包含一个main.rs文件
```rust
fn main() {
    println!("Hello, world!");
```
