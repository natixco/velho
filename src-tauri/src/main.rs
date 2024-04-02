#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};

use tauri::{CustomMenuItem, Manager, PhysicalPosition, SystemTray, SystemTrayMenu};

use crate::light_controller::LightController;
use crate::light_controller_wrapper::LightControllerWrapper;
use crate::light_storage::LightStorage;
use crate::socket_handler::SocketHandler;
use crate::storage::{AppInfo, Storage};

mod light;
mod storage;
mod commands;
mod socket_handler;
mod light_controller;
mod light_controller_wrapper;
mod light_storage;

fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quit", "Quit"));
    let system_tray = SystemTray::new()
        .with_menu(tray_menu);

    tauri::Builder::default()
        .setup(move |app| {
            let storage = Storage::new(AppInfo {
                handle: app.handle().clone(),
                config: app.config().clone(),
                package_info: app.package_info().clone(),
                env: app.env().clone(),
            })?;

            let mut socket_handler = SocketHandler::new()?;
            socket_handler.start_listen(app.handle().clone());

            app.manage(LightStorage::new(app.handle().clone(), storage));
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
        .system_tray(system_tray)
        .on_system_tray_event(|app_handle, event| match event {
            tauri::SystemTrayEvent::LeftClick { position, size, .. } => {
                let window = app_handle.get_window("main").unwrap();
                if !window.is_visible().unwrap() {
                    let window_size = window.outer_size().unwrap();
                    let physical_pos = PhysicalPosition {
                        x: position.x as i32 + (size.width as i32 / 2) - (window_size.width as i32 / 2),
                        y: position.y as i32 - window_size.height as i32,
                    };

                    let _ = window.set_position(tauri::Position::Physical(physical_pos));
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            tauri::SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    app_handle.exit(0);
                }
                _ => {}
            }
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(false) => {
                event.window().hide().unwrap();
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
