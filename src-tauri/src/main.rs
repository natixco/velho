#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::Manager;

use crate::event_handler::event_handler::EventHandler;
use crate::storage::{AppInfo, Storage};

mod device;
mod event_handler;
mod storage;
mod commands;

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let mut storage = Storage::new(AppInfo {
                handle: app.handle().clone(),
                config: app.config().clone(),
                package_info: app.package_info().clone(),
                env: app.env().clone(),
            })?;
            let event_handler = EventHandler::new(app.handle().clone())?;

            storage.load();
            app.manage(storage);

            event_handler.set_event_listeners();

            let mut socket_handler = event_handler.socket_handler.lock().unwrap();
            socket_handler.start_listen(app.handle().clone());
            socket_handler.start_auto_update();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::get_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}