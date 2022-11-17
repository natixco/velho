use std::fmt::Formatter;
use serde::{Serialize, Serializer};
use serde::ser::{SerializeStruct};

#[derive(Clone)]
pub struct Device {
  pub ip: String,
  pub mac: String,
  pub state: bool,
  pub scene_id: u64,
  pub temp: u64,
  pub dimming: u64,
}

impl Serialize for Device {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
  {
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
