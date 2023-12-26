use std::sync::Mutex;

use serde_json::Value;
use tauri::{AppHandle, Manager};

use crate::light::Light;
use crate::storage::Storage;

pub struct LightStorage {
    lights: Mutex<Vec<Light>>,
    app_handle: AppHandle,
    storage: Storage,
}

impl LightStorage {
    pub fn new(app_handle: AppHandle, storage: Storage) -> Self {
        Self {
            lights: Mutex::new(storage.load()),
            app_handle,
            storage,
        }
    }

    pub fn get_lights(&self) -> Vec<Light> {
        let lights = self.lights.lock().unwrap();
        lights.to_vec()
    }

    pub fn upsert_light(&self, mut light: Light) -> () {
        let mut lights = self.lights.lock().unwrap();
        let _light;
        if let Some(existing_light) = lights.iter_mut().find(|d| d.state.mac == light.state.mac) {
            existing_light.state = light.state;
            existing_light.available = true;
            _light = existing_light;
        } else {
            light.name = light.state.mac.clone();
            light.available = true;
            lights.push(light.clone());
            _light = &mut light;
        }

        self.app_handle.get_window("main").unwrap().emit("upsert_light", _light.clone()).unwrap();

        self.storage.save(lights.to_vec());
        drop(lights);
    }

    pub fn update_light(&self, mac: String, params: Value) -> bool {
        let mut lights = self.lights.lock().unwrap();

        if let Some(light) = lights.iter_mut().find(|d| d.state.mac == mac) {
            light.name = params.get("name")
                .and_then(Value::as_str)
                .map_or_else(|| light.state.mac.clone(), |s| s.to_string());

            return true;
        }

        self.storage.save(lights.to_vec());
        drop(lights);

        false
    }
}