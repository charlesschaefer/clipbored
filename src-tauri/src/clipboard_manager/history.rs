extern crate clipboard_master;
use clipboard_master::{CallbackResult, ClipboardHandler};

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;

pub struct ClipboardHistory(Mutex<VecDeque<String>>);

impl ClipboardHistory {
    pub fn new() -> Self {
        ClipboardHistory(Mutex::new(VecDeque::with_capacity(10)))
    }

    pub fn add_item(&self, item: String) {
        let mut history = self.0.lock().unwrap();
        // Remove item if it already exists to avoid duplicates
        history.retain(|x| x != &item);
        // Add new item to front
        history.push_front(item);
        // Keep only last 10 items
        while history.len() > 10 {
            history.pop_back();
        }
    }

    pub fn get_items(&self) -> Vec<String> {
        self.0.lock().unwrap().iter().cloned().collect()
    }
}

#[derive(Debug)]
pub struct Handler {
    app: Arc<Mutex<AppHandle>>,
}

impl Handler {
    pub fn new(app: Arc<Mutex<AppHandle>>) -> Self {
        Handler { app: app }
    }
}

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        let app = self.app.lock().unwrap();
        let clipboard = app.clipboard();
        let history = app.state::<ClipboardHistory>();

        if let Ok(text) = clipboard.read_text() {
            history.add_item(text.to_string());

            use crate::clipboard_manager::tray::setup_tray_menu;

            setup_tray_menu(&app, Some(true));
        }
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: std::io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}
