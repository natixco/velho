use std::sync::{Arc, Mutex};

use serde_json::{json, Value};

use crate::socket_handler::SocketHandler;

pub struct LightController {
    socket_handler: Arc<Mutex<SocketHandler>>,
}

impl LightController {
    pub fn new(socket_handler: Arc<Mutex<SocketHandler>>) -> Self {
        Self {
            socket_handler,
        }
    }

    pub fn refresh_devices(&self) -> () {
        self.socket_handler.lock().unwrap().broadcast_get_pilot();
    }

    pub fn set_pilot(&self, ip: String, params: Value) -> () {
        self.send_event("setPilot", ip, params);
    }

    fn send_event(&self, method: &str, device_ip: String, params: Value) -> () {
        let data = json!({
            "method": method,
            "params": params,
        });
        match self.socket_handler.lock() {
            Ok(handler) => {
                handler.send_data(data.to_string().as_bytes(), device_ip).unwrap();
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}