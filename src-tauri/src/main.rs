#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use std::fs;
// use std::env;

#[tauri::command]
fn rules() -> String {
    // let path = env::current_dir();
    // println!("The current directory is {}", path.expect("REASON").display());
    let result = fs::read_to_string("/usr/interparser.json").unwrap();
    return  result.into();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![rules])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
