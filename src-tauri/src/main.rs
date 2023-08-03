// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dev;
mod io;
mod mac_os;
mod windows;

use base64::{engine::general_purpose, Engine as _};
use image::DynamicImage;
use std::fs::OpenOptions;
use std::io::{prelude::*, Cursor};
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
async fn image_base64(path: String) -> Result<String, String> {
    const MAX_SIZE: u32 = 200;
    if let Ok(mut file) = OpenOptions::new()
        .read(true)
        .create(false)
        .write(false)
        .open(&path)
    {
        let mut file_content = Vec::new();
        if let Ok(_) = file.read_to_end(&mut file_content) {
            return if let Ok(image) = image::load_from_memory(&file_content) {
                let mut compress_content = Cursor::new(Vec::new());
                let resized: DynamicImage;
                let encoded: String;
                if image.height() > MAX_SIZE || image.width() > MAX_SIZE {
                    if image.height() > image.width() {
                        let ratio = image.width() as f64 / MAX_SIZE as f64;
                        let new_height = (image.height() as f64 * ratio) as u32;
                        resized = image.thumbnail(MAX_SIZE, new_height);
                    } else {
                        let ratio = image.height() as f64 / MAX_SIZE as f64;
                        let new_width = (image.width() as f64 * ratio) as u32;
                        resized = image.thumbnail(new_width, MAX_SIZE);
                    }
                    resized
                        .write_to(&mut compress_content, image::ImageOutputFormat::Jpeg(80))
                        .expect("ERR:COMPRESS_IMAGE");
                    encoded =
                        general_purpose::STANDARD_NO_PAD.encode(compress_content.into_inner());
                } else {
                    encoded = general_purpose::STANDARD_NO_PAD.encode(file_content);
                }
                Ok(encoded)
            } else {
                Err("ERR:LOAD_IMAGE".to_string())
            };
        }
    } else {
        return Err("ERR:OPEN_FILE".to_string());
    }
    Err("ERR:COMPRESS_IMAGE".to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            {
                mac_os::on_created(app, window);
            }

            #[cfg(target_os = "windows")]
            {
                windows::on_created(app, window)
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            dev::open_devtools,
            io::get_file,
            io::write_file,
            image_base64,
            windows::is_windows10
        ])
        .run(tauri::generate_context!())
        .unwrap();
}
