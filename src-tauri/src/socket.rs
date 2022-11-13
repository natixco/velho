use std::error::Error;
use std::fmt::Formatter;
use std::net::UdpSocket;
use std::thread;
use std::time::{Duration, Instant};
use serde::{Serialize, Serializer};
use serde::ser::{SerializeSeq, SerializeStruct};
use serde_json::{json, Value};

static LOCAL_ADDRESS: &str = "0.0.0.0";
static BROADCAST_ADDRESS: &str = "255.255.255.255";
static PORT: &str = "38899";

#[derive(Clone)]
pub struct Device {
  ip: String,
  mac: String,
  state: bool,
  scene_id: u64,
  temp: u64,
  dimming: u64,
}

impl Serialize for Device {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
  {
    // 3 is the number of fields in the struct.
    let mut state = serializer.serialize_struct("Device", 6)?;
    state.serialize_field("ip", &self.ip)?;
    state.serialize_field("mac", &self.mac)?;
    state.serialize_field("state", &self.state)?;
    state.serialize_field("scene_id", &self.scene_id)?;
    state.serialize_field("temp", &self.temp)?;
    state.serialize_field("dimming", &self.dimming)?;
    state.end()
  }
}


impl std::fmt::Display for Device {
  fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
    println!("Device: {} - Mac: {} - SceneId: {}", self.ip, self.mac, self.scene_id);
    Ok(())
  }
}

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
    let mut socket;
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

  pub fn discover(&self) -> std::io::Result<Vec<Device>> {
    let now = Instant::now();
    let cloned = self.socket.try_clone().unwrap();

    let thread_handle = thread::spawn(move || {
      let mut devices: Vec<Device> = Vec::new();
      loop {
        let elapsed_time = now.elapsed().as_micros();
        if elapsed_time > 5000 {
          return devices;
        }

        let mut buf: [u8; 1024] = [0; 1024];
        let result = cloned.recv_from(&mut buf);
        match result {
          Ok((number_of_bytes, src)) => {
            let data = String::from_utf8(buf[0..number_of_bytes].to_vec()).unwrap();
            println!("Data: {}", data);
            let mut object: Value = serde_json::from_str(&data).unwrap();
            if let Some(result) = object.get("result") {
              let _ = &devices.push(Device {
                ip: src.to_string(),
                mac: String::from(result.get("mac").unwrap().as_str().unwrap()),
                scene_id: result.get("sceneId").unwrap().as_u64().unwrap(),
                dimming: result.get("dimming").unwrap().as_u64().unwrap(),
                state: result.get("state").unwrap().as_bool().unwrap(),
                temp: result.get("temp").unwrap().as_u64().unwrap(),
              });
            }
          }
          Err(err) => panic!("Read error: {}", err)
        }
      }
    });

    let json = json!({
      "method": "getPilot",
      "params": {}
    });
    self.socket.send_to(json.to_string().as_bytes(), format!("{}:{}", BROADCAST_ADDRESS, PORT)).unwrap();
    Ok(thread_handle.join().unwrap())
  }

  pub fn set_state(&self, device_ip: String, params: String) -> () {
    let s: Value = serde_json::from_str(&params).unwrap();
    let json = json!({
      "method": "setState",
      "params": s,
    });
    self.socket.send_to(json.to_string().as_bytes(), device_ip).unwrap();
  }
}
