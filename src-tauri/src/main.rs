// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod image_manipulation;

use image_manipulation::{Image, ImageWrapper, ImageConverter, ImageToTextConverter, ImageScaleOptions};



#[tauri::command]
fn get_char_image(path: &str) -> String {
    if let Ok(mut image_wrapper) = ImageWrapper::from_path(path) {
        let mut converter = ImageToTextConverter { image_wrapper };
        return converter.convert();

    } else {
        let mut image_wrapper = ImageWrapper::new(10, 10);
        let mut converter = ImageToTextConverter { image_wrapper };
        return converter.convert();
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_char_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
