#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};

use tauri::Manager;

use crate::light_controller::LightController;
use crate::light_controller_wrapper::LightControllerWrapper;
use crate::socket_handler::SocketHandler;
use crate::storage::{AppInfo, Storage};

mod light;
mod storage;
mod commands;
mod socket_handler;
mod light_controller;
mod light_controller_wrapper;

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
            app.manage(LightControllerWrapper {
                controller: Arc::new(Mutex::new(LightController::new(Arc::new(Mutex::new(socket_handler))))),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_lights,
            commands::set_pilot,
            commands::update_light,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
