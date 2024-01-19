// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod image_manipulation;

use image_manipulation::{Image, ImageWrapper, convert_to_char_image, ImageScaleOptions};



#[tauri::command]
fn get_char_image(path: &str) -> Vec<Vec<char>> {
    if let Ok(mut image_wrapper) = ImageWrapper::from_path(path) {
        return convert_to_char_image(&mut image_wrapper, ImageScaleOptions::default());
    } else {
        let mut image_wrapper = ImageWrapper::new(10, 10);
        return convert_to_char_image(&mut image_wrapper, ImageScaleOptions::default());
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_char_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
