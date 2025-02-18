mod clipboard_manager;
mod commands;

use std::{str::FromStr, sync::{Arc, Mutex}};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

use clipboard_manager::{history::ClipboardHistory, tray::setup_tray_menu};
use tauri::{App, Manager};

use commands::{AppConfig, Bookmark};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut}; // Removed APP_CONFIG and BOOKMARKS

use std::fs;

fn load_file_configs(app: &App) -> (AppConfig, Vec<Bookmark>) {
    // Changed to return the values
    let save_path = app
        .path()
        .app_local_data_dir()
        .expect("Failed to get data directory"); // Moved inside the function
    let config_path = save_path.join("config.json");

    let app_config = match fs::read_to_string(&config_path) {
        Ok(contents) => {
            serde_json::from_str(&contents).unwrap_or_else(|_| {
                // Defaults
                AppConfig {
                    max_items: 10,
                    open_shortcut: "Ctrl+Shift+V".into(),
                    bookmark_shortcut: "Ctrl+Shift+B".into(),
                    start_minimized: false,
                }
            })
        }
        Err(_) => {
            // Defaults
            AppConfig {
                max_items: 10,
                open_shortcut: "Ctrl+Shift+V".into(),
                bookmark_shortcut: "Ctrl+Shift+B".into(),
                start_minimized: false,
            }
        }
    };

    let bookmark_path = save_path.join("bookmarks.json"); // Corrected path
    let bookmarks = match fs::read_to_string(&bookmark_path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| vec![]),
        Err(_) => {
            vec![]
        }
    };

    (app_config, bookmarks) // Return the loaded values
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![])
        ))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build()) // Add the global shortcut plugin
        .setup(|app: &mut App| {
            //// sets up the autostart function
            let autostart_manager = app.autolaunch();
            // Enable autostart
            let _ = autostart_manager.enable();

            //// sets up the managed state variables
            use crate::clipboard_manager::history::Handler;
            use clipboard_master::Master;

            app.manage(ClipboardHistory::new());

            // Load configs and manage state
            let (app_config, bookmarks) = load_file_configs(app);
            let config = app_config.clone();
            app.manage(Arc::new(Mutex::new(app_config)));
            app.manage(Arc::new(Mutex::new(bookmarks)));

            //// Sets up the tray menu
            setup_tray_menu(&app.handle(), None);

            //// Start a thread with the clipboard listener
            let app_handle = Arc::new(Mutex::new(app.handle().to_owned()));
            std::thread::spawn(move || {
                // Set up clipboard listener
                let mut master = Master::new(Handler::new(app_handle));
                master.run().expect("run monitor");
            });


            let global_shortcut_manager = app.global_shortcut();
            let open_shortcut = Shortcut::from_str(config.open_shortcut.as_str()).unwrap();
            let bookmark_shortcut = Shortcut::from_str(&config.bookmark_shortcut.as_str()).unwrap();

            global_shortcut_manager.on_shortcut(open_shortcut, clipboard_manager::handlers::open_shortcut_handler).unwrap();
            global_shortcut_manager.on_shortcut(bookmark_shortcut, clipboard_manager::handlers::bookmark_shortcut_handler).unwrap();

            //app.manage(global_shortcut_manager);


            //// Hides the window if that is the configuration
            if config.start_minimized {
                let _ = app.get_webview_window("main").unwrap().hide();
            }

            Ok(())
        })
        //.plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler!(
            commands::remove_bookmark,
            commands::get_bookmarks,
            commands::add_bookmark,
            commands::set_config,
            commands::get_config,
            commands::hide_window
        ))
        //.invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}