#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use serde_json::Value;
use tauri::Manager;
use window_shadows::set_shadow;
use window_vibrancy::apply_acrylic;

use crate::device::Device;
use crate::socket::SocketHandler;

mod device;
mod socket;

fn main() {
  let socket_handler = Arc::new(Mutex::new(SocketHandler::new().unwrap()));
  let devices: Vec<Device> = Vec::new();
  let (device_sender, device_receiver): (Sender<Device>, Receiver<Device>) = mpsc::channel();

  let thread_tx = device_sender.clone();
  socket_handler.lock().unwrap().discover(thread_tx);

  let s = Arc::clone(&socket_handler);
  let s2 = Arc::clone(&socket_handler);
  let mut thread_devices = devices.clone();
  tauri::Builder::default()
    .setup(|app| {
      let main_window = app.get_window("main").unwrap();

      #[cfg(target_os = "windows")]
      apply_acrylic(&main_window, Some((18, 18, 18, 180))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");
      set_shadow(&main_window, true).expect("Unsupported platform!");

      app.listen_global("set_pilot", move |event| {
        let payload = event.payload().unwrap();
        let object: Value = serde_json::from_str(payload).unwrap();
        let device_ip = String::from(object.get("deviceIp").unwrap().as_str().unwrap());
        s.lock().unwrap().set_pilot(device_ip, object.get("params").unwrap());
      });

      app.listen_global("set_state", move |event| {
        let payload = event.payload().unwrap();
        let object: Value = serde_json::from_str(payload).unwrap();
        let device_ip = String::from(object.get("deviceIp").unwrap().as_str().unwrap());
        let state = object.get("state").unwrap().as_bool().unwrap();
        s2.lock().unwrap().set_state(device_ip, state);
      });

      let thread_main_window = main_window.clone();
      thread::spawn(move || loop {
        let new_device = device_receiver.recv().unwrap();
        let new_device_copy = new_device.clone();
        match thread_devices.iter().position(|device| device.mac == new_device.mac) {
          Some(index) => {
            thread_devices[index] = new_device;
          },
          None => {
            thread_devices.push(new_device);
          }
        }
        thread_main_window.emit("device_discovery", new_device_copy).unwrap();
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
