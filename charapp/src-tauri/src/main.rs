// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 大概分为3层: tauri-app层 --> 中间适配过渡关联层 --> axum纯服务层

fn main() {
    charapp_lib::run()
}
