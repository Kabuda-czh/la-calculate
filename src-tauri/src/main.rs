// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod calculate;

use calculate::BuildParam;

#[tauri::command]
async fn build_calculate(build_param: BuildParam) -> (Vec<String>, Vec<String>) {
    println!("{:?}", build_param);
    calculate::calculate(build_param)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![build_calculate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
