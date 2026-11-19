#![allow(unused)]
#[doc(hidden)] pub mod errors;
#[doc(inline)] pub use errors::{Error, Result};
use serde::Serialize;
use tauri::{AppHandle, Emitter, EventTarget, ipc::Channel, tray::TrayIconBuilder};
use tauri_plugin_dialog::DialogExt;



#[tauri::command]
fn open_file(app: AppHandle) {
    app.dialog().file().pick_file(move |file_path| match file_path {
        Some(path) => {
            app.emit("file-selected", &path.to_string()).unwrap();
            println!("{path}");
        }
        None => {
            eprintln!("no path");
        }
    });
}
use iocore::Path;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
