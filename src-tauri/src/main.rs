#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::{mpsc};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use tauri::Manager;
use crate::device::Device;
use crate::socket::{SocketHandler};
use window_vibrancy::{apply_acrylic};
use window_shadows::set_shadow;

mod device;
mod socket;

fn main() {
  let socket_handler = SocketHandler::new().unwrap();
  let (tx, rx): (Sender<Device>, Receiver<Device>) = mpsc::channel();

  let thread_tx = tx.clone();
  socket_handler.discover(thread_tx);

  tauri::Builder::default()
    .setup(|app| {
      let main_window = app.get_window("main").unwrap();

      #[cfg(target_os = "windows")]
      apply_acrylic(&main_window, Some((18, 18, 18, 180))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");
      set_shadow(&main_window, true).expect("Unsupported platform!");

      app.listen_global("test_event", |event| {
        println!("Event: {:?}", event.payload());
      });

      let thread_main_window = main_window.clone();
      thread::spawn(move || loop {
        let new_devices = rx.recv().unwrap();
        thread_main_window.emit("device_discovery", new_devices).unwrap();
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// #[tauri::command]
// fn discover() -> Vec<Device> {
//   let mut state = state.inner().0.lock().unwrap();
//   match state.socket_handler.discover() {
//     Ok(devices) => {
//       state.devices = devices.clone();
//       devices
//     },
//     Err(_) => {
//       Vec::new()
//     },
//   }
// }

// #[tauri::command]
// fn set_state(device_ip: String, params: String, global_state: tauri::State<WizState>) -> () {
//   let mut _global_state = global_state.inner().0.lock().unwrap();
//   let _result = _global_state.socket_handler.set_state(device_ip, params);
//   // println!("Result: {}", result.unwrap());
//   // result.unwrap()
// }
//
// #[tauri::command]
// fn set_pilot(device_ip: String, params: String, global_state: tauri::State<WizState>) -> () {
//   let mut _global_state = global_state.inner().0.lock().unwrap();
//   let _result = _global_state.socket_handler.set_pilot(device_ip, params);
//   // println!("Result: {}", result.unwrap());
//   // result.unwrap()
// }
