#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{AppHandle, Builder, CustomMenuItem, Menu, Submenu, WindowBuilder, WindowUrl};

fn main() {
    // https://tauri.app/v1/guides/features/menu
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(close));
    let menu = Menu::new().add_submenu(submenu);

    Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "close" => {
                event.window().close().unwrap();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![greet, open_google])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
async fn open_google(app: AppHandle) {
	// TODO: with_ipc_handler???
    let common = include_str!("./common.js");
    WindowBuilder::new(
        &app,
        "exGoogle",
        WindowUrl::External("https://www.google.com/".parse().unwrap()),
    )
    .title("Google")
    .inner_size(1280.0, 720.0)
    .initialization_script(common)
    .build()
	.unwrap();
}

