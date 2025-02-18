#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use tauri::Manager;

use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub max_items: usize,
    pub open_shortcut: String,
    pub bookmark_shortcut: String,
    pub start_minimized: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bookmark {
    pub content: String,
}
// Placeholder for your actual config data
// pub static mut APP_CONFIG: Option<AppConfig> = None;  REMOVED
// pub static mut BOOKMARKS: Option<Vec<Bookmark>> = None; REMOVED

#[tauri::command]
pub fn get_config(app: tauri::AppHandle) -> Option<AppConfig> {
    // unsafe { APP_CONFIG.clone() }  REMOVED
    let config = app.state::<Arc<Mutex<AppConfig>>>().inner().lock().unwrap();
    let conf = config.clone();
    Some(conf)
}

#[tauri::command]
pub fn set_config(app: tauri::AppHandle, config: AppConfig) -> Result<(), String> {
    println!("saving the config {:?}", config);

    // Save the config to a file using app.app_handle()
    if let Err(e) = save_config_to_file(&app, &config) {
        return Err(format!("Failed to save config: {}", e));
    }

    let mut app_config = app.state::<Arc<Mutex<AppConfig>>>().inner().lock().unwrap();
    *app_config = config;


    Ok(())
}

#[tauri::command]
pub fn get_bookmarks(app: tauri::AppHandle) -> Option<Vec<Bookmark>>{
    // unsafe { BOOKMARKS.clone() } REMOVED
    let bookmarks = app.state::<Arc<Mutex<Vec<Bookmark>>>>();
    let bm = bookmarks.lock().unwrap();
    Some(bm.to_vec())
}

#[tauri::command]
pub fn remove_bookmark(app: tauri::AppHandle, index: usize) -> Result<(), String> {
    let mut bookmarks = app.state::<Arc<Mutex<Vec<Bookmark>>>>().inner().lock().unwrap();
        if index < bookmarks.len() {
            bookmarks.remove(index);
            // Save after removing
            if let Err(e) = save_bookmark_to_file(&app, &bookmarks) {
                return Err(format!("Failed to save bookmarks after removal: {}", e));
            }
            Ok(())
        } else {
            Err("Index out of bounds".into())
        }
}


#[tauri::command]
pub fn add_bookmark(app: tauri::AppHandle, content: String) -> Result<(), String> {
    let mut bookmarks = app.state::<Arc<Mutex<Vec<Bookmark>>>>().inner().lock().unwrap();
    bookmarks.push(Bookmark { content: content.clone() });

    // Save after adding
    if let Err(e) = save_bookmark_to_file(&app, &bookmarks) {
        return Err(format!("Failed to save bookmarks after adding: {}", e));
    }

    Ok(())
}


#[tauri::command]
pub fn hide_window(app: tauri::AppHandle) {
    app.get_webview_window("main").unwrap().hide().unwrap();
}

// Add a helper function to handle the file saving.
fn save_config_to_file(app: &tauri::AppHandle, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = app.path().app_local_data_dir().unwrap();
    
    
    let config_file = config_dir.join("config.json");

    let serialized_config = serde_json::to_string(config)?;

    let mut file = File::create(config_file)?;
    file.write_all(serialized_config.as_bytes())?;

    Ok(())
}

fn save_bookmark_to_file(app: &tauri::AppHandle, bookmarks: &Vec<Bookmark>) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = app.path().app_local_data_dir().unwrap();

    let bookmark_file = config_dir.join("bookmarks.json");

    let serialized_bookmark = serde_json::to_string(bookmarks)?;

    let mut file = File::create(bookmark_file)?;
    file.write_all(serialized_bookmark.as_bytes())?;

    Ok(())
}