use std::sync::{Arc, Mutex};

use serde_json::{json, Value};

use crate::socket_handler::SocketHandler;

pub struct DeviceController {
    socket_handler: Arc<Mutex<SocketHandler>>,
}

impl DeviceController {
    pub fn new(socket_handler: Arc<Mutex<SocketHandler>>) -> Self {
        Self {
            socket_handler
        }
    }

    pub fn refresh_devices(&self) -> () {
        self.socket_handler.lock().unwrap().broadcast_get_pilot();
    }

    pub fn set_state(&self, device_ip: String, state: bool) -> () {
        self.send_event("setState", device_ip, json!({"state": state}));
    }

    pub fn set_pilot(&self, device_ip: String, params: Value) -> () {
        self.send_event("setPilot", device_ip, params);
    }

    fn send_event(&self, method: &str, device_ip: String, params: Value) -> () {
        let data = json!({
            "method": method,
            "params": params,
        });
        match self.socket_handler.lock() {
            Ok(handler) => {
                handler.socket.send_to(data.to_string().as_bytes(), device_ip).unwrap();
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}