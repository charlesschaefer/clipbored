use tauri::{
    menu::MenuEvent, AppHandle, Manager
};

use tauri_plugin_clipboard_manager::ClipboardExt;
use clipboard_master::Master;
use crate::clipboard_manager::history::Handler;



use super::history::ClipboardHistory;

pub fn handle_tray_event(
    app: &AppHandle,
    event: MenuEvent
) {
    let clipboard = app.clipboard();

    let history = app.state::<ClipboardHistory>();
    dbg!("Vamos tratar os eventos de trayicon");
    match event.id.0.as_str() {
                "quit" => std::process::exit(0),
                item_id if item_id.starts_with("item_") => {
                    if let Ok(index) = item_id[5..].parse::<usize>() {
                        let items = history.get_items();
                        if let Some(text) = items.get(index) {
                            clipboard.write_text::<String>(text.clone()).unwrap();
                        }
                    }
                }
                _ => {}
    }
}


pub fn setup_clipboard_listener(app: &mut tauri::App) {
    let mut master = Master::new(Handler::new(app.handle()));
    master.run().expect("run monitor");
}


