use std::fmt;
use std::io;

#[derive(Debug)]
pub struct StatsError {
    pub message: String
}

// 实现Display trait
impl fmt::Display for StatsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Error > {
            write!(f, "StatsError: {}", self.message)
        }
    }
}

// 从&str 转换为 StatsError
impl From<&str> for StatsError {
    fn from(s: &str) -> Self {
        StatsError {
            message: s.to_string()
        }
    }
}

// IO 错误转换为 StatsError
impl From<io::Error> for StatsError {
    fn from(e: io::Error) -> Self {
        StatsError {
            message: e.to_string()
        }
    }
}

// usize 转为u32  错误检查
impl From<std::num::TryFromIntError> for StatsError {
    fn from(_e: std::num::TryFromIntError) -> Self {
        StatsError {
            message: "Number conversion error".to_string()
        }
    }
}