// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // setup 完整流程
    // 运行js代码, rust代码, shell代码
    // on_window_events
    // build实例 tauri::Builder::default() --> beforeCommand --> 关联前端构建资源
    tauri::Builder::default()
        // 添加插件
        .plugin(tauri_plugin_opener::init())
        // 前端路由事件
        .invoke_handler(tauri::generate_handler![greet])
        // 执行上下文
        .run(tauri::generate_context!())
        // 错误捕获
        .expect("error while running tauri application");
}
