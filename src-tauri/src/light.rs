use serde_json::Value;
use std::fmt;

use crate::constants::*;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LightState {
    pub ip: String,
    pub mac: String,
    pub state: bool,
    pub scene_id: u64,
    pub temp: u64,
    pub dimming: u64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Light {
    pub name: String,
    pub available: bool,
    pub state: LightState,
}

impl fmt::Display for Light {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Light {{ ip: {}, mac: {}, name: {}, available: {} }}",
               self.state.ip, self.state.mac, self.name, self.available)
    }
}

impl Light {
    pub fn new(ip: String, params: &Value) -> Self {
        let mac = params.get(MAC_FIELD)
            .and_then(Value::as_str)
            .unwrap_or(&ip)
            .to_string();

        let scene_id = params.get(SCENE_ID_FIELD)
            .and_then(Value::as_u64)
            .unwrap_or(0);

        let dimming = params.get(DIMMING_FIELD)
            .and_then(Value::as_u64)
            .unwrap_or(100);

        let state = params.get(STATE_FIELD)
            .and_then(Value::as_bool)
            .unwrap_or(false);

        let temp = params.get(TEMP_FIELD)
            .and_then(Value::as_u64)
            .unwrap_or(2700);

        Self {
            name: String::new(),
            available: false,
            state: LightState {
                ip,
                mac,
                scene_id,
                dimming,
                state,
                temp,
            },
        }
    }

    pub fn with_default_name(mut self) -> Self {
        if self.name.is_empty() {
            self.name = self.state.mac.clone();
        }
        self
    }

    pub fn update_state(mut self, params: &Value) -> Self {
        if let Some(scene_id) = params.get(SCENE_ID_FIELD).and_then(Value::as_u64) {
            self.state.scene_id = scene_id;
        }

        if let Some(dimming) = params.get(DIMMING_FIELD).and_then(Value::as_u64) {
            self.state.dimming = dimming;
        }

        if let Some(state) = params.get(STATE_FIELD).and_then(Value::as_bool) {
            self.state.state = state;
        }

        if let Some(temp) = params.get(TEMP_FIELD).and_then(Value::as_u64) {
            self.state.temp = temp;
        }

        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn mark_available(mut self) -> Self {
        self.available = true;
        self
    }

    pub fn mark_unavailable(mut self) -> Self {
        self.available = false;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_light_creation_with_complete_data() {
        let ip = "192.168.1.100".to_string();
        let params = json!({
            "mac": "AA:BB:CC:DD:EE:FF",
            "sceneId": 1,
            "dimming": 75,
            "state": true,
            "temp": 3000
        });

        let light = Light::new(ip.clone(), &params);

        assert_eq!(light.state.ip, ip);
        assert_eq!(light.state.mac, "AA:BB:CC:DD:EE:FF");
        assert_eq!(light.state.scene_id, 1);
        assert_eq!(light.state.dimming, 75);
        assert_eq!(light.state.state, true);
        assert_eq!(light.state.temp, 3000);
        assert_eq!(light.name, "");
        assert_eq!(light.available, false);
    }

    #[test]
    fn test_light_creation_with_missing_data() {
        let ip = "192.168.1.100".to_string();
        let params = json!({
            "mac": "AA:BB:CC:DD:EE:FF"
        });

        let light = Light::new(ip.clone(), &params);

        assert_eq!(light.state.ip, ip);
        assert_eq!(light.state.mac, "AA:BB:CC:DD:EE:FF");
        assert_eq!(light.state.scene_id, 0);
        assert_eq!(light.state.dimming, 100);
        assert_eq!(light.state.state, false);
        assert_eq!(light.state.temp, 2700);
    }

    #[test]
    fn test_light_creation_with_missing_mac() {
        let ip = "192.168.1.100".to_string();
        let params = json!({});

        let light = Light::new(ip.clone(), &params);

        assert_eq!(light.state.ip, ip);
        assert_eq!(light.state.mac, ip);
        assert_eq!(light.state.scene_id, 0);
        assert_eq!(light.state.dimming, 100);
        assert_eq!(light.state.state, false);
        assert_eq!(light.state.temp, 2700);
    }

    #[test]
    fn test_light_builder_methods() {
        let ip = "192.168.1.100".to_string();
        let params = json!({});

        let light = Light::new(ip, &params)
            .with_default_name()
            .with_name("Living Room Light".to_string())
            .mark_available();

        assert_eq!(light.name, "Living Room Light");
        assert_eq!(light.available, true);
    }

    #[test]
    fn test_light_state_update() {
        let ip = "192.168.1.100".to_string();
        let params = json!({});

        let light = Light::new(ip, &params);

        let update_params = json!({
            "sceneId": 5,
            "dimming": 50,
            "state": true
        });

        let updated_light = light.update_state(&update_params);

        assert_eq!(updated_light.state.scene_id, 5);
        assert_eq!(updated_light.state.dimming, 50);
        assert_eq!(updated_light.state.state, true);
        assert_eq!(updated_light.state.temp, 2700);
    }
}
