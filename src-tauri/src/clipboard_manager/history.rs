extern crate clipboard_master;
use clipboard_master::{CallbackResult, ClipboardHandler};

use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use tauri::{AppHandle, Manager, Emitter};
use tauri_plugin_clipboard_manager::ClipboardExt;

pub struct ClipboardHistory(RwLock<VecDeque<String>>, usize);


impl ClipboardHistory {
    pub fn new(limit: usize) -> Self {
        ClipboardHistory(RwLock::new(VecDeque::with_capacity(limit)), limit)
    }

    pub fn add_item(&self, item: String) {
        let mut history = self.0.write().unwrap();
        // Remove item if it already exists to avoid duplicates
        history.retain(|x| x != &item);
        // Add new item to front
        history.push_front(item);
        // Keep only last 10 items
        while history.len() > self.1 {
            history.pop_back();
        }
    }

    pub fn get_items(&self) -> Vec<String> {
        self.0.read().unwrap().iter().cloned().collect()
    }

    // Add remove_item method
    pub fn remove_item(&self, item: String) {
        let mut history = self.0.write().unwrap();
        history.retain(|x| x != &item);
    }

    pub fn change_limit(&mut self, limit: usize) {
        let old_itens = self.get_items();

        self.0 = RwLock::new(VecDeque::with_capacity(limit));
        self.1 = limit;

        let mut i = 0;
        for item in old_itens {
            if (i + 1) > limit {
                break;
            }

            self.add_item(item);

            i += 1;
        }
    }
}

#[derive(Debug)]
pub struct Handler {
    app: Arc<RwLock<AppHandle>>,
}

impl Handler {
    pub fn new(app: Arc<RwLock<AppHandle>>) -> Self {
        Handler { app: app }
    }
}

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        let app = self.app.read().unwrap();
        let clipboard = app.clipboard();
        let history = app.state::<Arc<RwLock<ClipboardHistory>>>().inner().write().unwrap();

        if let Ok(text) = clipboard.read_text() {
            history.add_item(text.to_string());
            drop(history);

            use crate::clipboard_manager::tray::setup_tray_menu;

            setup_tray_menu(&app, Some(true));

            // Emit the event here!
            if let Err(e) = app.emit_to("main", "clipboard-updated", ()) {
                eprintln!("Error emitting clipboard-updated: {}", e);
            } else {
                dbg!("Emitou o clipboard-updated");
            }
        }
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: std::io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}
