use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use serde_json::{json, Value};
use tauri::{AppHandle, Manager};

use crate::constants::*;
use crate::light::Light;
use crate::light_storage::LightStorage;

pub struct SocketHandler {
    socket: UdpSocket,
    is_listening: Arc<AtomicBool>,
}

impl SocketHandler {
    pub fn new() -> std::io::Result<Self> {
        let socket = UdpSocket::bind(format!("{}:{}", LOCAL_ADDRESS, PORT))?;
        socket.set_broadcast(true)?;

        Ok(Self {
            socket,
            is_listening: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn start_listen(&mut self, app_handle: AppHandle) -> Result<(), String> {
        if self.is_listening.load(Ordering::Acquire) {
            return Ok(());
        }

        self.is_listening.store(true, Ordering::Release);

        let thread_socket = self.socket.try_clone()
            .map_err(|e| format!("Failed to clone socket: {}", e))?;

        let is_listening = Arc::clone(&self.is_listening);

        thread::spawn(move || {
            let mut buf = [0u8; BUFFER_SIZE];

            while is_listening.load(Ordering::Acquire) {
                match thread_socket.recv_from(&mut buf) {
                    Ok((bytes_received, src)) => {
                        if let Ok(data) = String::from_utf8(buf[..bytes_received].to_vec()) {
                            if let Err(e) = Self::process_received_data(&data, &src, &app_handle) {
                                eprintln!("Error processing received data: {}", e);
                            }
                        } else {
                            eprintln!("Received invalid UTF-8 data from {}", src);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving data: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    pub fn stop_listen(&mut self) {
        self.is_listening.store(false, Ordering::Release);
    }

    pub fn broadcast_get_pilot(&self) -> Result<(), String> {
        let data = json!({
            METHOD_FIELD: GET_PILOT_METHOD,
            PARAMS_FIELD: {}
        });

        let message = data.to_string();
        let addr = format!("{}:{}", BROADCAST_ADDRESS, PORT);

        self.send_data(message.as_bytes(), &addr)
            .map_err(|e| format!("Failed to broadcast getPilot: {}", e))?;

        Ok(())
    }

    pub fn send_data(&self, data: &[u8], addr: &str) -> std::io::Result<usize> {
        self.socket.send_to(data, addr)
    }

    fn process_received_data(data: &str, src: &SocketAddr, app_handle: &AppHandle) -> Result<(), String> {
        let object: Value = serde_json::from_str(data)
            .map_err(|e| format!("Failed to parse JSON data: {}", e))?;

        if let Some(result) = object.get(RESULT_FIELD) {
            if result.get(SUCCESS_FIELD).is_none() {
                let light = Light::new(src.ip().to_string(), result)
                    .with_default_name()
                    .mark_available();

                app_handle.state::<LightStorage>().upsert_light(light)
                    .map_err(|e| format!("Failed to upsert light: {}", e))?;
            }
        }

        Ok(())
    }
}

impl Drop for SocketHandler {
    fn drop(&mut self) {
        self.stop_listen();
    }
}
