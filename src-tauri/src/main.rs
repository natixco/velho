#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};

use tauri::Manager;

use crate::device_controller::DeviceController;
use crate::device_controller_wrapper::DeviceControllerWrapper;
use crate::socket_handler::SocketHandler;
use crate::storage::{AppInfo, Storage};

mod device;
mod storage;
mod commands;
mod device_controller;
mod socket_handler;
mod device_controller_wrapper;

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let mut storage = Storage::new(AppInfo {
                handle: app.handle().clone(),
                config: app.config().clone(),
                package_info: app.package_info().clone(),
                env: app.env().clone(),
            })?;
            storage.load();

            let mut socket_handler = SocketHandler::new()?;
            socket_handler.start_listen(app.handle().clone());

            app.manage(storage);
            app.manage(DeviceControllerWrapper {
                controller: Arc::new(Mutex::new(DeviceController::new(Arc::new(Mutex::new(socket_handler))))),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_devices,
            commands::set_pilot,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
