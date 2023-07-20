// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::alloc::System;

#[global_allocator]
static A: System = System;

use base64::{engine::general_purpose, Engine as _};
use image::DynamicImage;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{prelude::*, Cursor};
use std::path::Path;
use tauri::Manager;
use window_shadows::set_shadow;
use window_vibrancy::{apply_acrylic, apply_vibrancy, NSVisualEffectMaterial};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn get_file(path: String, def: String) -> Result<String, ()> {
    let parent_path = Path::new(&path).parent().expect("ERR:PARENT_PATH");
    if !parent_path.exists() {
        create_dir_all(parent_path).expect("ERR:CREATE_DIR");
    }
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .expect("ERR:OPEN_FILE");
    let mut file_content = String::new();
    return if let Ok(_) = file.read_to_string(&mut file_content) {
        if file_content.is_empty() {
            if let Err(_) = file.write_all(&def.as_bytes()) {
                eprint!("ERR:WRIT_FILE")
            } else {
                return Ok(def);
            }
        }
        Ok(file_content)
    } else {
        Err(())
    };
}

#[tauri::command]
fn write_file(path: String, content: String) -> Result<(), ()> {
    let parent_path = Path::new(&path).parent().expect("ERR:PARENT_PATH");
    if !parent_path.exists() {
        create_dir_all(parent_path).expect("ERR:CREATE_DIR");
    }
    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .open(&path)
        .expect("ERR:OPEN_FILE");
    if let Err(_) = file.write_all(&content.as_bytes()) {
        return Err(());
    }
    Ok(())
}

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
    let info = os_info::get();
    println!("OS version: {}", info.version());
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            #[cfg(debug_assertions)]
            {
                window.open_devtools();
            }

            #[cfg(target_os = "macos")]
            {
                apply_vibrancy(
                    &window,
                    NSVisualEffectMaterial::WindowBackground,
                    None,
                    None,
                )
                .unwrap();
            }

            #[cfg(target_os = "windows")]
            {
                window.set_decorations(true).unwrap();
                apply_acrylic(&window, Some((14, 14, 14, 20))).unwrap();
                // apply_mica(&window, None).unwrap();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_file, write_file, image_base64])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
