use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

use serde_json::{json, Value};
use tauri::{AppHandle, Manager};

use crate::device::Device;
use crate::storage::Storage;

static LOCAL_ADDRESS: &str = "0.0.0.0";
static BROADCAST_ADDRESS: &str = "255.255.255.255";
static PORT: &str = "38899";

pub struct SocketHandler {
    pub socket: UdpSocket,
    started_listen: bool,
    started_auto_update: bool,
}

impl SocketHandler {
    pub fn new() -> std::io::Result<Self> {
        let socket = UdpSocket::bind(format!("{}:{}", LOCAL_ADDRESS, PORT))?;
        socket.set_broadcast(true)?;
        Ok(Self {
            socket,
            started_listen: false,
            started_auto_update: false,
        })
    }

    pub fn start_listen(&mut self, app_handle: AppHandle) -> () {
        if self.started_listen {
            return;
        }

        self.started_listen = true;

        let thread_receiver_socket = self.socket.try_clone().unwrap();
        thread::spawn(move || loop {
            let mut buf: [u8; 1024] = [0; 1024];
            let (number_of_bytes, src) = thread_receiver_socket.recv_from(&mut buf).unwrap();
            let data = String::from_utf8(buf[0..number_of_bytes].to_vec()).unwrap();
            println!("Received data: {}", data);

            let object: Value = serde_json::from_str(&data).unwrap();

            if let Some(result) = object.get("result") {
                if result.get("success").is_none() {
                    let new_device = Device::new(src.to_string(), result);
                    app_handle.state::<Storage>().upsert_device(new_device);
                }
            }
        });
    }

    pub fn start_auto_update(&mut self) -> () {
        if self.started_auto_update {
            return;
        }

        self.started_auto_update = true;

        let thread_sender_socket = self.socket.try_clone().unwrap();
        thread::spawn(move || loop {
            let data = json!({
                "method": "getPilot",
                "params": {}
            });
            thread_sender_socket.send_to(data.to_string().as_bytes(), format!("{}:{}", BROADCAST_ADDRESS, PORT)).unwrap();
            thread::sleep(Duration::from_secs(5));
        });
    }
}
