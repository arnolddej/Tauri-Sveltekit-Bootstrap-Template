// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serial2_tokio::SerialPort;

use std::path::PathBuf;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_available_serialports() -> Vec<PathBuf> {
    let av_ports = SerialPort::available_ports();
    println!("{:?}", av_ports);
    

    av_ports.unwrap()

    // Ok(SerialPort::available_ports()?)
    //   todo!();
}

fn main() {
    let av_ports = SerialPort::available_ports();
    println!("{:?}", av_ports);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, list_available_serialports])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
