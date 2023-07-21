use std::fs::OpenOptions;

use std::fs::create_dir_all;

use std::io::Read;
use std::io::Write;

use std::path::Path;

#[tauri::command]
pub(crate) fn get_file(path: String, def: String) -> Result<String, ()> {
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
pub(crate) fn write_file(path: String, content: String) -> Result<(), ()> {
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
