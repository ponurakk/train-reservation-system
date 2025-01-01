use std::env::current_dir;

use chrono::Local;
use gbrouting::csa::{gtfs::parse_gtfs, routing::ConnectionScanCore};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_stops() {
    // println!("{:#?}", current_dir());
    let gtfs = gbrouting::gtfs::load("../gtfs/pkpic.zip").unwrap();
    let cs_data = parse_gtfs(
        &gtfs,
        Local::now().naive_local().date(),
        true,
        1000.0,
        0.6,
        false,
    )
    .unwrap();

    let cs_core = ConnectionScanCore::new(&cs_data);
    // println!("{:#?}", cs_core.get_stops());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_stops])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
