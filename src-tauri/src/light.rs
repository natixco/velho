use serde_json::Value;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct LightState {
    pub ip: String,
    pub mac: String,
    pub state: bool,
    pub scene_id: u64,
    pub temp: u64,
    pub dimming: u64,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Light {
    pub name: String,
    pub available: bool,
    pub state: LightState,
}

impl std::fmt::Display for Light {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("ip: {} - mac: {}", self.state.ip, self.state.mac);
        Ok(())
    }
}

impl Light {
    pub fn new(ip: String, params: &Value) -> Self {
        Self {
            name: String::new(),
            available: false,
            state: LightState {
                ip,
                mac: String::from(params.get("mac").unwrap().as_str().unwrap()),
                scene_id: params.get("sceneId").unwrap().as_u64().unwrap(),
                dimming: params.get("dimming").unwrap().as_u64().unwrap(),
                state: params.get("state").unwrap().as_bool().unwrap(),
                temp: params.get("temp").unwrap().as_u64().unwrap(),
            }
        }
    }
}
