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
关于[toml](https://toml.io/en/)
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
}
```

## 构建
```bash
cargo build
    Compiling hello_cargo v0.1.0 (C:\Users\86186\rust-practice\tools\cargo\hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.84s

```

这个时候 会在项目文件夹中生成一个target目录
## 运行
```bash
cargo run
   # 如果已经cargo build过了，下面的就不会出现
    Compiling hello_cargo v0.1.0 (C:\Users\86186\rust-practice\tools\cargo\hello_cargo)
     Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\hello_cargo.exe`
Hello, world!
```

如果只是单纯的监查代码是否可以正常执行，但不生产可执行文件
## 检查
```bash
cargo check
 Checking hello_cargo v0.1.0 (C:\Users\86186\rust-practice\tools\cargo\hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.57s

# 如果把程序改成错误的，再次执行，会出现下面的错误
 cargo check
    Checking hello_cargo v0.1.0 (C:\Users\86186\rust-practice\tools\cargo\hello_cargo)
error[E0765]: unterminated double quote string
 --> src\main.rs:2:14
  |
2 |       println!("Hello, world!);
  |  ______________^
3 | | }
  | |__^

For more information about this error, try `rustc --explain E0765`.
error: could not compile `hello_cargo` due to previous error

```
这个时候，你去看target目录里面debug是没有生成执行文件的
