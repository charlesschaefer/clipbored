use tauri::{
    include_image, menu::{CheckMenuItemBuilder, IconMenuItemBuilder, MenuBuilder, MenuItem, MenuItemBuilder}, tray::TrayIconBuilder, AppHandle, Manager
};

use super::{
    handlers::{
        handle_tray_menu_event, handle_tray_icon_event
    }, 
    history::ClipboardHistory
};
use crate::commands::Bookmark;
use std::sync::{Arc, RwLock};

pub fn setup_tray_menu(app_handle: &AppHandle, update_tray: Option<bool>) {
    //let menu = Menu::new(app_handle).unwrap();
    let mut menu = MenuBuilder::new(app_handle);
    let items: Vec<String>;
    if let Some(_) = update_tray {
        items = app_handle.state::<Arc<RwLock<ClipboardHistory>>>().read().unwrap().get_items();
    } else {
        items = Vec::new();
    }

    // Load the bookmark icon *once*
    //let bookmark_icon = app_handle.asset_resolver().get("assets/bookmark.ico".to_string()).expect("Failed to load bookmark icon");
    let bookmark_icon = include_image!("../src/assets/bookmark.ico");

    // Create bookmark menu items
    let bookmarks = app_handle.state::<Arc<RwLock<Vec<Bookmark>>>>().inner().read().unwrap().to_vec();
    let bookmark_menu_items = bookmarks_as_menu_items_for_tray(&bookmarks);

    // Create initial empty menu
    let menu_items = history_as_menu_items_for_tray(&items);

    menu = menu.item(&MenuItemBuilder::new("BOOKMARKS")
        .enabled(false)
        .build(app_handle)
        .unwrap()
    );

    // Combine bookmarks and history (bookmarks first)
    for (id, text) in bookmark_menu_items {
        let item = IconMenuItemBuilder::new(text)
            .icon(bookmark_icon.clone())
            .id(id)
            .build(app_handle).unwrap();
        
        menu = menu.item(&item);
    }

    // Add a separator
    menu = menu.separator();
    menu = menu.item(&MenuItemBuilder::new("CLIPBOARD")
        .enabled(false)
        .build(app_handle)
        .unwrap()
    );

    for (id, text) in menu_items {
        // let checked = if (i as usize) == (items.len() - 1) { true } else { false };
        let item = CheckMenuItemBuilder::new(text)
            .id(id)
            .checked(false)
            .enabled(true)
            .build(app_handle)
            .unwrap();
        
        menu = menu.item(&item);
        // menu.append(&item).unwrap();
    }

    // let image: Image<'_>;
    // let asset = match app_handle.asset_resolver().get("icon.png".to_string()) {
    //     Some(img) => {
    //         dbg!("Deu bom na imagem");
    //         let mut bytes: [u8] = [];
    //         bytes.copy_from_slice(img.bytes());
    //         let mut b2 = bytes.clone();
    //         b2.copy_from_slice(bytes);
    //         image = Image::new(b2, 136, 136);
    //     },
    //     _ => {
    //         dbg!("deu ruim demais");
    //         image = Image::new(&[0u8;0], 0, 0);
    //     }
    // };
    // //let image = Image::new(&asset, 136, 136);
    // let test = IconMenuItem::with_id(app_handle, "img", "Image", true, Some(image), None::<&str>).unwrap();
    // Add separator and quit
    let quit_item = MenuItem::with_id(app_handle, "quit", "Quit", true, None::<&str>).unwrap();
    let show_item = MenuItem::with_id(app_handle, "show", "Settings", true, None::<&str>).unwrap();
    // menu.append(&test).unwrap();
    // menu.append(&show_item).unwrap();
    // menu.append(&quit_item).unwrap();

    let built_menu = menu.separator()
        .item(&show_item)
        .item(&quit_item).build().unwrap();
     

    // holds the menu in the AppHandle's internal state so we can access it to show when user passes the mouse 
    // over the tray icon
    //app_handle.set_menu(built_menu.clone()).unwrap();

    if let Some(_) = update_tray {
        // Update the tray menu
        if let Some(tray) = app_handle.tray_by_id("main") {
            let _ = tray.set_menu(Some(built_menu.clone()));
        }
    } else {
        // Create tray icon
        TrayIconBuilder::with_id("main")
            .menu(&built_menu)
            .on_menu_event(move |app_handle, event| {
                handle_tray_menu_event(app_handle, event);
            })
            .icon(app_handle.default_window_icon().unwrap().clone())
            .on_tray_icon_event(move |tray_icon, event| {
                handle_tray_icon_event(tray_icon, event);
            })
            .build(app_handle)
            .unwrap();
    }

    //// Sets up the event that prevents the window from closing and hides it instead
    let window = app_handle.get_webview_window("main").unwrap();
    let window_hider = window.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            window_hider.hide().unwrap();
        }
    });

}

pub fn history_as_menu_items_for_tray(history: &Vec<String>) -> Vec<(String, String)> {
    let mut menu_items = Vec::new();

    // Add clipboard history items
    for (index, item) in history.iter().enumerate() {
        let display_text = if item.len() > 30 {
            format!("{}...", &item[..30])
        } else {
            item.clone()
        };

        menu_items.push((format!("item_{}", index), display_text));
    }

    menu_items
}

pub fn bookmarks_as_menu_items_for_tray(bookmarks: &Vec<Bookmark>) -> Vec<(String, String)> {
    let mut menu_items = Vec::new();

    // Add clipboard history items
    for (index, item) in bookmarks.iter().enumerate() {
        let display_text = if item.content.len() > 30 {
            format!("{}...", &item.content[..30])
        } else {
            item.content.clone()
        };

        menu_items.push((format!("item_bm_{}", index), display_text));
    }

    menu_items
}
//     let mut menu_items = Vec::new();

//     for (index, bookmark) in bookmarks.iter().enumerate() {
//         let display_text = if bookmark.content.len() > 30 {
//             format!("{}...", &bookmark.content[..30])
//         } else {
//             bookmark.content.clone()
//         };

//         // Use IconMenuItemBuilder for items with icons
//         let item = IconMenuItem::new(
//             display_text,
//             true,
//             Some(icon.clone()),
//             Some(format!("bookmark_{}", index)), // Use a different prefix for bookmark IDs
//         );
//         menu_items.push(item);
//     }
//     menu_items
// }
