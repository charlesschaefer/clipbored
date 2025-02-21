#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use tauri::{Emitter, Manager};
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, RwLock};

use crate::clipboard_manager::history::ClipboardHistory;
use crate::clipboard_manager::tray::setup_tray_menu;

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

#[tauri::command]
pub fn get_config(app: tauri::AppHandle) -> Option<AppConfig> {
    let config = app.state::<Arc<RwLock<AppConfig>>>().inner().read().unwrap();
    Some(config.clone())
}

#[tauri::command]
pub fn set_config(app: tauri::AppHandle, config: AppConfig) -> Result<(), String> {
    // Save the config to a file using app.app_handle()
    if let Err(e) = save_config_to_file(&app, &config) {
        return Err(format!("Failed to save config: {}", e));
    }

    if let Err(error) = crate::clipboard_manager::handlers::register_keyboard_shortcuts(&app, &config) {
        return Err(error.to_string());
    }

    //let mut app_config = app.state::<Arc<Mutex<AppConfig>>>().inner().lock().unwrap();
    //let mut app_config = app.state::<Arc<Mutex<AppConfig>>>().get_mut().unwrap();
    //app_config.clone_from(&config);//.clone_into(app_config);
    let mut app_config = app.state::<Arc<RwLock<AppConfig>>>().inner().write().unwrap();
    *app_config = config;

    let mut history = app.state::<Arc<RwLock<ClipboardHistory>>>().inner().write().unwrap();
    history.change_limit(app_config.max_items);

    drop(history);
    drop(app_config);
    std::thread::spawn(move || {
        setup_tray_menu(app.app_handle(), Some(true));
    });

    Ok(())
}

#[tauri::command]
pub fn get_bookmarks(app: tauri::AppHandle) -> Option<Vec<Bookmark>> {
    let bookmarks = app.state::<Arc<RwLock<Vec<Bookmark>>>>().inner().read().unwrap();
    //Some(bm.to_vec())
    Some(bookmarks.clone())
}

#[tauri::command]
pub fn remove_bookmark(app: tauri::AppHandle, index: usize) -> Result<(), String> {
    dbg!("Entramos no remove_bookmark");
    let mut bookmarks = app
        .state::<Arc<RwLock<Vec<Bookmark>>>>()
        .inner()
        .write()
        .unwrap();
    if index < bookmarks.len() {
        bookmarks.remove(index);
        // Save after removing
        if let Err(e) = save_bookmark_to_file(&app, &bookmarks) {
            return Err(format!("Failed to save bookmarks after removal: {}", e));
        }
        
        drop(bookmarks);
        std::thread::spawn(move || {
            setup_tray_menu(app.app_handle(), Some(true));
        });
        
        Ok(())
    } else {
        Err("Index out of bounds".into())
    }
}

#[tauri::command]
pub fn add_bookmark(app: tauri::AppHandle, content: String) -> Result<(), String> {
    dbg!("Entramos no add_bookmark");
    let mut bookmarks = app
        .state::<Arc<RwLock<Vec<Bookmark>>>>()
        .inner()
        .write()
        .unwrap();
    bookmarks.push(Bookmark {
        content: content.clone(),
    });

    // Save after adding
    if let Err(e) = save_bookmark_to_file(&app, &bookmarks) {
        return Err(format!("Failed to save bookmarks after adding: {}", e));
    }

    drop(bookmarks);
    std::thread::spawn(move || {
        setup_tray_menu(app.app_handle(), Some(true));
    });

    Ok(())
}

#[tauri::command]
pub fn hide_window(app: tauri::AppHandle) {
    app.get_webview_window("main").unwrap().hide().unwrap();
}

// Add a helper function to handle the file saving.
fn save_config_to_file(
    app: &tauri::AppHandle,
    config: &AppConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = app.path().app_local_data_dir().unwrap();

    let config_file = config_dir.join("config.json");

    let serialized_config = serde_json::to_string(config)?;

    let mut file = File::create(config_file)?;
    file.write_all(serialized_config.as_bytes())?;

    Ok(())
}

fn save_bookmark_to_file(
    app: &tauri::AppHandle,
    bookmarks: &Vec<Bookmark>,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = app.path().app_local_data_dir().unwrap();

    let bookmark_file = config_dir.join("bookmarks.json");

    let serialized_bookmark = serde_json::to_string(bookmarks)?;

    let mut file = File::create(bookmark_file)?;
    file.write_all(serialized_bookmark.as_bytes())?;

    Ok(())
}

#[tauri::command]
pub fn get_clipboard_items(app: tauri::AppHandle) -> Vec<String> {
    let history = app.state::<Arc<RwLock<ClipboardHistory>>>().inner().write().unwrap();
    history.get_items()
}

// New command: toggle_bookmark (add and remove)
#[tauri::command]
pub fn toggle_bookmark(app: tauri::AppHandle, content: String) -> Result<(), String> {
    // 1. Check if it's already a bookmark.
    let mut bookmarks = app.state::<Arc<RwLock<Vec<Bookmark>>>>().inner().write().unwrap();
    let bookmark_index = bookmarks.iter().position(|b| b.content == content);

    if let Some(index) = bookmark_index {
        // 2. If it exists, remove it from bookmarks.
        bookmarks.remove(index);
        if let Err(e) = save_bookmark_to_file(&app, &bookmarks) {
            return Err(format!("Failed to save bookmarks after removal: {}", e));
        } else {
            println!("Deu certo desregistrar o bookmark");
        }
    } else {
        // 3. If it doesn't exist, add it to bookmarks.
        bookmarks.push(Bookmark { content: content.clone() });
        if let Err(e) = save_bookmark_to_file(&app, &bookmarks) {
            return Err(format!("Failed to save bookmarks after adding: {}", e));
        } else {
            println!("Deu certo registrar o bookmark");
        }
    }
    
    drop(bookmarks);
    app.emit_to("main", "bookmarks-updated", ()).unwrap();
    std::thread::spawn(move || {
        setup_tray_menu(app.app_handle(), Some(true));
    });

    Ok(())
}

// New command: delete_clipboard_item
#[tauri::command]
pub fn delete_clipboard_item(app: tauri::AppHandle, item: String) -> Result<(), String> {
    let history = app.state::<Arc<RwLock<ClipboardHistory>>>().inner().write().unwrap();
    history.remove_item(item);

    drop(history);
    std::thread::spawn(move || {
        setup_tray_menu(app.app_handle(), Some(true));
    });

    Ok(())
}
