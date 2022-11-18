use std::fmt::Formatter;

use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
pub struct Device {
  pub ip: String,
  pub mac: String,
  pub state: bool,
  pub scene_id: u64,
  pub temp: u64,
  pub dimming: u64,
}

impl std::fmt::Display for Device {
  fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
    println!("Device: {} - Mac: {} - SceneId: {}", self.ip, self.mac, self.scene_id);
    Ok(())
  }
}

impl Device {
  pub fn new(ip: String, params: &Value) -> std::io::Result<Self> {
    Ok(Self {
      ip,
      mac: String::from(params.get("mac").unwrap().as_str().unwrap()),
      scene_id: params.get("sceneId").unwrap().as_u64().unwrap(),
      dimming: params.get("dimming").unwrap().as_u64().unwrap(),
      state: params.get("state").unwrap().as_bool().unwrap(),
      temp: params.get("temp").unwrap().as_u64().unwrap(),
    })
  }
}
