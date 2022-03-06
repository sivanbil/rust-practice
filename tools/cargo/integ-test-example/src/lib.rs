//! 这是一个库
//! 包含进程ID的获取函数
use std::process;

/// 获取进程ID
pub fn get_process_id() -> u32 {
    process::id()
}