mod clipboard_manager;

use std::sync::{Arc, Mutex};

use clipboard_manager::{
    history::ClipboardHistory,
    tray::setup_tray_menu,
};
use tauri::{App, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app: &mut App| {
            use clipboard_master::Master;
            use crate::clipboard_manager::history::Handler;

            app.manage(ClipboardHistory::new());
            
            setup_tray_menu(&app.handle(), None);

            let app_handle = Arc::new(Mutex::new(app.handle().to_owned()));
            std::thread::spawn(move || {
                // Set up clipboard listener
                let mut master = Master::new(Handler::new(app_handle));
                master.run().expect("run monitor");
            });

            Ok(())
        })
        //.plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        //.invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
