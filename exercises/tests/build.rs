//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取当前 Unix 时间戳
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 设置环境变量 `TEST_FOO`，用于 tests7.rs
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 启用 pass feature，确保 tests8.rs 能通过
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
