// 在Windows上的发布版本中防止额外的控制台窗口，请勿删除！
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod apk_parser;
mod commands;

use std::fs;
use std::path::Path;
use std::env;
use std::time::Instant;

/// 初始化应用程序所需的资源
fn init_resources() -> Result<(), Box<dyn std::error::Error>> {
    println!("INFO: 正在初始化资源...");
    let start_time = Instant::now();
    
    // 如果不存在，创建resources目录
    let resources_dir = "resources";
    if !Path::new(resources_dir).exists() {
        match fs::create_dir_all(resources_dir) {
            Ok(_) => println!("INFO: 创建resources目录成功"),
            Err(e) => println!("WARNING: 创建resources目录失败: {}, 将继续执行", e),
        }
    } else {
        println!("INFO: resources目录已存在");
    }
    
    // 检查aapt2.exe是否存在
    let aapt2_path = format!("{}/aapt2.exe", resources_dir);
    if !Path::new(&aapt2_path).exists() {
        println!("ERROR: 未找到aapt2.exe，请确保aapt2.exe已放置在resources目录下");
        return Err("aapt2.exe not found".into());
    }
    
    let elapsed = start_time.elapsed();
    println!("INFO: 资源初始化完成，耗时: {:?}", elapsed);
    Ok(())
}

fn main() {
    // 初始化资源
    if let Err(e) = init_resources() {
        eprintln!("ERROR: 初始化资源时出错: {}", e);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::parse_apk,
            commands::get_app_info,
            // commands::parse_apk_data,
            commands::select_apk_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
