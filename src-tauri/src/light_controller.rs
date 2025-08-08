use std::sync::{Arc, Mutex};

use serde_json::{json, Value};

use crate::constants::*;
use crate::socket_handler::SocketHandler;

pub struct LightController {
    socket_handler: Arc<Mutex<SocketHandler>>,
}

impl LightController {
    pub fn new(socket_handler: Arc<Mutex<SocketHandler>>) -> Self {
        Self { socket_handler }
    }

    pub fn refresh_devices(&self) -> Result<(), String> {
        self.socket_handler
            .lock()
            .map_err(|e| format!("Failed to acquire socket handler lock: {}", e))?
            .broadcast_get_pilot()
    }

    pub fn set_pilot(&self, ip: String, params: Value) -> Result<(), String> {
        self.send_event(SET_PILOT_METHOD, ip, params)
    }

    fn send_event(&self, method: &str, device_ip: String, params: Value) -> Result<(), String> {
        let data = json!({
            METHOD_FIELD: method,
            PARAMS_FIELD: params,
        });

        let message = data.to_string();
        let addr = format!("{}:{}", device_ip, PORT);

        self.socket_handler
            .lock()
            .map_err(|e| format!("Failed to acquire socket handler lock: {}", e))?
            .send_data(message.as_bytes(), &addr)
            .map_err(|e| format!("Failed to send event to {}: {}", device_ip, e))?;

        Ok(())
    }
}