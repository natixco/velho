use std::sync::{Arc, Mutex};

use serde_json::Value;
use tauri::{AppHandle, Manager};

use crate::event_handler::device_controller::DeviceController;
use crate::event_handler::socket_handler::SocketHandler;

pub struct EventHandler {
    app_handle: AppHandle,
    pub socket_handler: Arc<Mutex<SocketHandler>>,
    device_controller: Arc<Mutex<DeviceController>>,
}

#[derive(Debug, serde::Deserialize)]
struct SetStatePayload {
    device_ip: String,
    state: bool,
}

#[derive(Debug, serde::Deserialize)]
struct SetPilotPayload {
    device_ip: String,
    params: Value,
}

impl EventHandler {
    pub fn new(app_handle: AppHandle) -> std::io::Result<Self> {
        let socket_handler = SocketHandler::new()?;
        let arc_mutex_socket_handler = Arc::new(Mutex::new(socket_handler));
        let device_controller = DeviceController::new(arc_mutex_socket_handler.clone())?;

        Ok(Self {
            app_handle,
            socket_handler: arc_mutex_socket_handler.clone(),
            device_controller: Arc::new(Mutex::new(device_controller)),
        })
    }

    pub fn set_event_listeners(&self) -> () {
        let a1 = self.device_controller.clone();
        self.app_handle.listen_global("set_state", move |event| {
            let object: SetStatePayload = serde_json::from_str(event.payload().unwrap()).unwrap();
            a1.lock().unwrap().set_state(object.device_ip, object.state);
        });

        let a2 = self.device_controller.clone();
        self.app_handle.listen_global("set_pilot", move |event| {
            let object: SetPilotPayload = serde_json::from_str(event.payload().unwrap()).unwrap();
            a2.lock().unwrap().set_pilot(object.device_ip, object.params);
        });
    }
}