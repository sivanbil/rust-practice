use std::convert::TryFrom;
use std::fs;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::fs::DirEntry;
use super::errors::StatsError;

#[derive(Debug)]
pub struct SrcStats {
    // 文件数量
    pub number_of_files: u32,
    // 代码行数
    pub loc: u32,
    // 注释行数
    pub comments: u32,
    // 空行数
    pub blanks: u32,

}

// 统计给定目录的rs文件的状况
pub fn get_summary_src_stats(in_dir: &Path) -> Result<SrcStats, StatsError> {
    let mut total_loc = 0;
    let mut total_comments = 0;
    let mut total_blanks = 0;
    let mut dir_entries: Vec<PathBuf> = vec![in_dir.to_path_buf()];
    let mut file_entries: Vec<DirEntry> = vec![];

    // 递归遍历目录
    while let Some(entry) = dir_entries.pop() {
        for inner_entry in fs::read_dir(&entry)? {
            if let Ok(inner_entry) = inner_entry {
                if inner_entry.path().is_dir() {
                    dir_entries.push(inner_entry.path());
                } else {
                   if entry.path().extension() == Some(OsStr::new("rs")) {
                       file_entries.push(inner_entry);
                   }
                }
            }
        }
    }

}

