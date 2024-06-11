// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod calculate;

use calculate::{BuildParam, CalculateResult};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// remember to call `.manage(MyState::default())`
#[tauri::command]
fn calculate(build_param: BuildParam) -> CalculateResult {
    println!("{:?}", build_param);
    calculate::calculate(build_param)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, calculate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
