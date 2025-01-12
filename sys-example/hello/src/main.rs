//! 这是一个库
//! 包含与进程处理相关的函数
//! 可使用执行这些任务更加方便

use mylib::hello_from_lib;
use chrono::Utc;
use std::process;

fn main() {
    println!("Going to call hello_from_lib from main.rs");
    hello_from_lib("Rust system programming");

    println!("Current time: {}", Utc::now());

    println!("{}", get_process_id());
}

/// 获取当前进程的ID
/// 它返回一个非零的值
fn get_process_id() -> u32 {
    process::id()
}

#[test] // carge test
#[ignore] // cargo test --ignored
fn test_get_process_id() {
    assert!(get_process_id() > 0);
}
