use std::net::UdpSocket;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use serde_json::{json, Value};

use crate::device::Device;

static LOCAL_ADDRESS: &str = "0.0.0.0";
static BROADCAST_ADDRESS: &str = "255.255.255.255";
static PORT: &str = "38899";

pub struct SocketHandler {
  socket: UdpSocket,
}

impl Default for SocketHandler {
  fn default() -> Self {
    Self { socket: UdpSocket::bind(format!("{}:{}", LOCAL_ADDRESS, PORT)).unwrap() }
  }
}

impl SocketHandler {
  pub fn new() -> std::io::Result<Self> {
    let attempt = UdpSocket::bind(format!("{}:{}", LOCAL_ADDRESS, PORT));
    let socket;
    match attempt {
      Ok(_socket) => {
        socket = _socket;
      },
      Err(err) => panic!("Could not bind: {}", err),
    }

    socket.set_broadcast(true).unwrap();

    Ok(Self {
      socket
    })
  }

  pub fn discover(&self, tx: Sender<Device>) -> () {
    let thread_receiver_socket = self.socket.try_clone().unwrap();
    thread::spawn(move || loop {
      let mut buf: [u8; 1024] = [0; 1024];
      let result = thread_receiver_socket.recv_from(&mut buf);
      match result {
        Ok((number_of_bytes, src)) => {
          let data = String::from_utf8(buf[0..number_of_bytes].to_vec()).unwrap();
          println!("Data: {}", data);
          let object: Value = serde_json::from_str(&data).unwrap();

          if let Some(result) = object.get("result") {
            if let None = result.get("success") {
              let device = Device::new(src.to_string(), result).unwrap();
              tx.send(device).unwrap();
            }
          }
        }
        Err(err) => panic!("Read error: {}", err)
      }
    });

    let thread_sender_socket = self.socket.try_clone().unwrap();
    thread::spawn(move || loop {
      let json = json!({
        "method": "getPilot",
        "params": {}
      });
      thread_sender_socket.send_to(json.to_string().as_bytes(), format!("{}:{}", BROADCAST_ADDRESS, PORT)).unwrap();
      thread::sleep(Duration::from_secs(5));
    });
  }

  pub fn set_state(&self, device_ip: String, state: bool) -> () {
    let json = json!({
      "method": "setState",
      "params": {
        "state": state
      },
    });
    self.socket.send_to(json.to_string().as_bytes(), device_ip).unwrap();
  }

  pub fn set_pilot(&self, device_ip: String, params: &Value) -> () {
    let json = json!({
      "method": "setPilot",
      "params": params,
    });
    self.socket.send_to(json.to_string().as_bytes(), device_ip).unwrap();
  }
}
