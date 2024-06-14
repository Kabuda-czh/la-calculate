// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod calculate;

use calculate::type_struct::{
    CalculatePageParam, CalculatePriceParam, CalculatePriceResult, CalculateResult,
};

#[tauri::command]
async fn calculate_build(
    build_param: CalculatePageParam,
    artifact_check: bool,
) -> Result<CalculateResult, ()> {
    let calculate_result = calculate::calculate_build_fn(build_param, artifact_check).await;
    Ok(calculate_result)
}

#[tauri::command]
async fn calculate_price(
    param: (Vec<CalculatePriceParam>, Vec<&str>),
) -> Result<Vec<CalculatePriceResult>, ()> {
    let calculate_price_result = calculate::calculate_price_fn(param).await;
    Ok(calculate_price_result)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calculate_build, calculate_price])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
