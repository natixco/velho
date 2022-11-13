use std::error::Error;
use std::fmt::Formatter;
use std::net::UdpSocket;
use std::thread;
use std::time::{Duration, Instant};
use serde_json::Value;

static LOCAL_ADDRESS: &str = "0.0.0.0";
static BROADCAST_ADDRESS: &str = "255.255.255.255";
static PORT: &str = "38899";

pub struct Device {
  ip: String,
  mac: String,
  state: bool,
  scene_id: u64,
  temp: u64,
  dimming: u64,
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
    let mut devices: Vec<Device> = Vec::new();

    let handle = thread::spawn(move || {
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
    println!("Send data");
    self.socket.send_to(&"{\"method\": \"getPilot\", \"params\": {}}".as_bytes().to_vec(), "255.255.255.255:38899").unwrap();
    let r = handle.join().unwrap();

    Ok(r)
  }
}
