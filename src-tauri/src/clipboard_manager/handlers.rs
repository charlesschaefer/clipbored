use std::{str::FromStr, sync::{Arc, RwLock}};
use tauri::{menu::MenuEvent, tray::{MouseButton, TrayIcon, TrayIconEvent}, AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut}; // Removed APP_CONFIG and BOOKMARKS
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::commands::{AppConfig, Bookmark};
use super::history::ClipboardHistory;

pub fn handle_tray_menu_event(app: &AppHandle, event: MenuEvent) {
    let clipboard = app.clipboard();

    
    match event.id.0.as_str() {
        "quit" => std::process::exit(0),
        "show" => app.get_webview_window("main").unwrap().show().unwrap(),
        item_id if item_id.starts_with("item_bm_") => {
            dbg!("Vem do bookmark");
            let bookmark_reader = app.state::<Arc<RwLock<Vec<Bookmark>>>>();
            let bookmarks = bookmark_reader.read().unwrap();
            if let Ok(index) = item_id[8..].parse::<usize>() {
                if let Some(text) = bookmarks.get(index) {
                    clipboard.write_text::<String>(text.clone().content).unwrap();
                    let _ = paste_text();
                }
            }
        },
        item_id if item_id.starts_with("item_") => {
            dbg!("Vem do clipboard");
            let history_reader = app.state::<Arc<RwLock<ClipboardHistory>>>();
            let history = history_reader.read().unwrap();
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
    use std::{time, thread};
    let send = |event: &EventType| -> Result<_, rdev::SimulateError> {
        simulate(event).unwrap();
        thread::sleep(time::Duration::from_millis(10));
        Ok(())
    };

    #[cfg(windows)] {
        // try to go back to the background window
        send(&EventType::KeyPress(Key::Alt))?;
        send(&EventType::KeyPress(Key::Escape))?;

        send(&EventType::KeyRelease(Key::Escape))?;
        send(&EventType::KeyRelease(Key::Alt))?;
    }

    // try pasting the text
    #[cfg(target_os = "linux")] {
        send(&EventType::KeyPress(Key::ShiftRight))?;
    }
    send(&EventType::KeyPress(Key::ControlLeft))?;
    send(&EventType::KeyPress(Key::KeyV))?;

    send(&EventType::KeyRelease(Key::KeyV))?;
    send(&EventType::KeyRelease(Key::ControlLeft))?;
    #[cfg(target_os = "linux")] {
        send(&EventType::KeyRelease(Key::ShiftRight))?;
    }

    Ok(())
}


pub fn handle_tray_icon_event(tray_icon: &TrayIcon, event: tauri::tray::TrayIconEvent) {
    match event {
        TrayIconEvent::Click {
            id: _,
            position: _,
            rect: _,
            button,
            button_state: _,
        } => match button {
            MouseButton::Left => {
                dbg!("system tray received a left click");
                let window = tray_icon.app_handle().get_webview_window("main").unwrap();
                let menu = tray_icon.app_handle().menu().unwrap();
                window.popup_menu(&menu).unwrap();
                if !window.is_visible().unwrap() {
                    window.show().unwrap();
                }
                //window.show().unwrap();
            }
            _ => {
                dbg!("system tray received a middle or right click");
            }
        },
        // TrayIconEvent::Enter { id: _, .. } => {
        //     dbg!("system tray received a mouse enter ");
        //     let window = tray_icon.app_handle().get_webview_window("main").unwrap();
        //     let menu = tray_icon.app_handle().menu();

        //     match menu {
        //         Some(menu) => {
        //             dbg!("Tem um menu no tray.app_handle()");
        //             window.popup_menu(&menu).unwrap();
        //         },
        //         None => {
        //             dbg!("Não tem um menu no tray.app_handle()");
        //         }
        //     }
        // },
        // TrayIconEvent::Leave { id: _, .. } => {
        //     dbg!("system tray received a mouse leave ");
        //     let window = tray_icon.app_handle().get_webview_window("main").unwrap();
        //     window.hide_menu().unwrap();
        // },
        _ => {}
    }
}

pub fn register_keyboard_shortcuts(app: &AppHandle, config: &AppConfig) -> Result<(), tauri_plugin_global_shortcut::Error>{
    let open_key = config.open_shortcut.as_str().replace("Meta", "Super");
    let bookmark_key = config.bookmark_shortcut.as_str().replace("Meta", "Super");

    let global_shortcut_manager = app.global_shortcut();
    let open_shortcut = Shortcut::from_str(&open_key).unwrap();
    let bookmark_shortcut = Shortcut::from_str(&bookmark_key).unwrap();
    // Unregister old shortcuts
    if global_shortcut_manager. is_registered(open_shortcut) {
        global_shortcut_manager.unregister(open_shortcut).unwrap();
    }
    if global_shortcut_manager.is_registered(bookmark_shortcut) {
        global_shortcut_manager.unregister(bookmark_shortcut).unwrap();
    }

    global_shortcut_manager.on_shortcut(open_shortcut, crate::clipboard_manager::handlers::open_shortcut_handler)?;
    global_shortcut_manager.on_shortcut(bookmark_shortcut, crate::clipboard_manager::handlers::bookmark_shortcut_handler)?;

    Ok(())
}

pub fn open_shortcut_handler<T, U>(app: &'_ AppHandle, _: &'_ T, _: U) {
    app.get_webview_window("main").unwrap().show().unwrap();
}

pub fn bookmark_shortcut_handler<T, U>(app: &'_ AppHandle, _: &'_ T, _: U) {
    let history_reader = app.state::<Arc<RwLock<ClipboardHistory>>>();
    let history = history_reader.read().unwrap();
    let items = history.get_items();

    drop(history);
    if let Some(last_item) = items.first() {
        let bookmarks = app.state::<Arc<RwLock<Vec<Bookmark>>>>().inner();
        let bm = bookmarks.write().unwrap();
        // Check if item is already bookmarked
        if let Some(index) = bm.iter().position(|b| b.content == *last_item) {
            drop(bm);
            let _ = crate::commands::remove_bookmark(app.to_owned(), index);
        } else {
            drop(bm);
            let _ = crate::commands::add_bookmark(app.to_owned(), last_item.clone());
        }
    }
}
