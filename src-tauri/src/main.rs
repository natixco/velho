#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};

use tauri::Manager;

use crate::light_controller::LightController;
use crate::light_controller_wrapper::LightControllerWrapper;
use crate::light_storage::LightStorage;
use crate::socket_handler::SocketHandler;
use crate::storage::{AppInfo, Storage};

mod constants;
mod light;
mod storage;
mod commands;
mod socket_handler;
mod light_controller;
mod light_controller_wrapper;
mod light_storage;

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let storage = Storage::new(AppInfo {
                handle: app.handle().clone(),
                config: app.config().clone(),
                package_info: app.package_info().clone(),
                env: app.env().clone(),
            })?;

            let mut socket_handler = SocketHandler::new()?;
            socket_handler.start_listen(app.handle().clone())?;

            let light_storage = LightStorage::new(app.handle().clone(), storage);
            let light_controller = LightController::new(Arc::new(Mutex::new(socket_handler)));
            let light_controller_wrapper = LightControllerWrapper::new(light_controller);

            app.manage(light_storage);
            app.manage(light_controller_wrapper);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_lights,
            commands::set_pilot,
            commands::update_light,
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_, event| match event {
            _ => {}
        });
}
