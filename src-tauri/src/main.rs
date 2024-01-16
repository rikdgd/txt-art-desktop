// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod image_manipulation;

use std::error::Error;
use image_manipulation::{Image, ImageWrapper, convert_to_char_image};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_char_image(path: &str) -> Vec<Vec<char>> {
    if let Ok(image_wrapper) = ImageWrapper::from_path(path) {
        return convert_to_char_image(image_wrapper);
    } else {
        let image_wrapper = ImageWrapper::new(10, 10);
        return convert_to_char_image(image_wrapper);
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
