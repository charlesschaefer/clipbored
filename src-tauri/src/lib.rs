mod clipboard_manager;

use clipboard_manager::{
    handlers::setup_clipboard_listener,
    history::ClipboardHistory,
    tray::setup_tray_menu,
};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(move |app| {
            app.manage(ClipboardHistory::new());
            
            setup_tray_menu(&app.handle(), None);

            // Set up clipboard listener
            setup_clipboard_listener(app);

            Ok(())
        })
        //.plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        //.invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
