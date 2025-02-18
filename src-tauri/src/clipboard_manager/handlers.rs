use tauri::{
    menu::MenuEvent, AppHandle, Manager
};

use tauri_plugin_clipboard_manager::ClipboardExt;

use super::history::ClipboardHistory;

pub fn handle_tray_event(
    app: &AppHandle,
    event: MenuEvent
) {
    let clipboard = app.clipboard();

    let history = app.state::<ClipboardHistory>();
    match event.id.0.as_str() {
                "quit" => std::process::exit(0),
                "show" => app.get_webview_window("main").unwrap().show().unwrap(),
                item_id if item_id.starts_with("item_") => {
                    if let Ok(index) = item_id[5..].parse::<usize>() {
                        let items = history.get_items();
                        if let Some(text) = items.get(index) {
                            println!("Text we got: {:?}", text);
                            clipboard.write_text::<String>(text.clone()).unwrap();
                            paste_text();
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