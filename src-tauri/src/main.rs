#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::{Mutex};
use tauri::Manager;
use crate::device::Device;
use crate::socket::{SocketHandler, test};
use window_vibrancy::{apply_blur, apply_acrylic, apply_vibrancy, NSVisualEffectMaterial};
use window_shadows::set_shadow;

mod device;
mod socket;

struct WizState(Mutex<InnerState>);

#[derive(Default)]
struct InnerState {
  socket_handler: SocketHandler,
  devices: Vec<Device>,
}

fn main() {
  tauri::Builder::default()
    .manage(WizState(Mutex::from(InnerState {
      devices: Vec::new(),
      socket_handler: SocketHandler::new().unwrap(),
    })))
    .setup(|app| {
      let main_window = app.get_window("main").unwrap();

      #[cfg(target_os = "windows")]
      apply_acrylic(&main_window, Some((18, 18, 18, 180))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");

      set_shadow(&main_window, true).expect("Unsupported platform!");

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![discover, set_state, set_pilot])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn discover(state: tauri::State<WizState>) -> Vec<Device> {
  let mut state = state.inner().0.lock().unwrap();
  match state.socket_handler.discover() {
    Ok(devices) => {
      state.devices = devices.clone();
      devices
    },
    Err(_) => {
      Vec::new()
    },
  }
}

#[tauri::command]
fn set_state(device_ip: String, params: String, global_state: tauri::State<WizState>) -> () {
  let mut _global_state = global_state.inner().0.lock().unwrap();
  let _result = _global_state.socket_handler.set_state(device_ip, params);
  // println!("Result: {}", result.unwrap());
  // result.unwrap()
}

#[tauri::command]
fn set_pilot(device_ip: String, params: String, global_state: tauri::State<WizState>) -> () {
  let mut _global_state = global_state.inner().0.lock().unwrap();
  let _result = _global_state.socket_handler.set_pilot(device_ip, params);
  // println!("Result: {}", result.unwrap());
  // result.unwrap()
}
