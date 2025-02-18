use std::sync::{Arc, Mutex};

use tauri::{menu::MenuEvent, AppHandle, Manager};

use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::commands::Bookmark;

use super::history::ClipboardHistory;

pub fn handle_tray_event(app: &AppHandle, event: MenuEvent) {
    let clipboard = app.clipboard();

    let history = app.state::<ClipboardHistory>();
    match event.id.0.as_str() {
        "quit" => std::process::exit(0),
        "show" => app.get_webview_window("main").unwrap().show().unwrap(),
        item_id if item_id.starts_with("item_") => {
            if let Ok(index) = item_id[5..].parse::<usize>() {
                let items = history.get_items();
                if let Some(text) = items.get(index) {
                    clipboard.write_text::<String>(text.clone()).unwrap();
                    let _ = paste_text();
                }
            }
        }
        _ => {}
    }
}

pub fn paste_text() -> Result<(), rdev::SimulateError> {
    use rdev::{simulate, EventType, Key};

    // Simulate pressing CTRL
    simulate(&EventType::KeyPress(Key::ControlLeft))?;
    // Simulate pressing SHIFT
    simulate(&EventType::KeyPress(Key::ShiftLeft))?;
    // Simulate pressing V
    simulate(&EventType::KeyPress(Key::KeyV))?;
    // Simulate releasing V
    simulate(&EventType::KeyRelease(Key::KeyV))?;
    // Simulate releasing SHIFT
    simulate(&EventType::KeyRelease(Key::ShiftLeft))?;
    // Simulate releasing CTRL
    simulate(&EventType::KeyRelease(Key::ControlLeft))?;

    Ok(())
}


pub fn open_shortcut_handler<T, U>(app: &'_ AppHandle, _: &'_ T, _: U) {
    app.get_webview_window("main").unwrap().show().unwrap();
}

pub fn bookmark_shortcut_handler<T, U>(app: &'_ AppHandle, _: &'_ T, _: U) {
    let history = app.state::<ClipboardHistory>();
    let items = history.get_items();
    if let Some(last_item) = items.first() {
        let bookmarks = app.state::<Arc<Mutex<Vec<Bookmark>>>>();
        let bm = bookmarks.lock().unwrap();
        // Check if item is already bookmarked
        if let Some(index) = bm.iter().position(|b| b.content == *last_item) {
            let _ = crate::commands::remove_bookmark(app.to_owned(), index);
        } else {
            let _ = crate::commands::add_bookmark(app.to_owned(), last_item.clone());
        }
    }
}