use std::sync::Mutex;

use serde_json::Value;
use tauri::{AppHandle, Manager};

use crate::constants::*;
use crate::light::Light;
use crate::storage::Storage;

pub struct LightStorage {
    lights: Mutex<Vec<Light>>,
    app_handle: AppHandle,
    storage: Storage,
}

impl LightStorage {
    pub fn new(app_handle: AppHandle, storage: Storage) -> Self {
        let mut lights = storage.load().unwrap_or_else(|e| {
            eprintln!("Failed to load lights from storage: {}", e);
            Vec::new()
        });

        for light in &mut lights {
            *light = light.clone().mark_unavailable();
        }

        Self {
            lights: Mutex::new(lights),
            app_handle,
            storage,
        }
    }

    pub fn get_lights(&self) -> Result<Vec<Light>, String> {
        self.lights
            .lock()
            .map_err(|e| format!("Failed to acquire lights lock: {}", e))
            .map(|lights| lights.clone())
    }

    pub fn upsert_light(&self, light: Light) -> Result<(), String> {
        let mut lights = self.lights
            .lock()
            .map_err(|e| format!("Failed to acquire lights lock: {}", e))?;

        if let Some(existing_light) = lights.iter_mut().find(|l| l.state.mac == light.state.mac) {
            let existing_light_name = existing_light.name.clone();
            *existing_light = light.with_name(existing_light_name);
            self.emit_light_event(existing_light.clone())?;
        } else {
            lights.push(light.clone());
            self.emit_light_event(light)?;
        }

        self.storage.save(&lights)
            .map_err(|e| format!("Failed to save lights: {}", e))?;

        Ok(())
    }

    pub fn update_light(&self, mac: String, params: Value) -> Result<bool, String> {
        let mut lights = self.lights
            .lock()
            .map_err(|e| format!("Failed to acquire lights lock: {}", e))?;

        if let Some(light) = lights.iter_mut().find(|l| l.state.mac == mac) {
            if let Some(name) = params.get(NAME_FIELD).and_then(Value::as_str) {
                light.name = name.to_string();
            }

            *light = light.clone().update_state(&params);

            self.storage.save(&lights)
                .map_err(|e| format!("Failed to save lights: {}", e))?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn emit_light_event(&self, light: Light) -> Result<(), String> {
        if let Some(window) = self.app_handle.get_window(MAIN_WINDOW_NAME) {
            window.emit(UPSERT_LIGHT_EVENT, light)
                .map_err(|e| format!("Failed to emit light event: {}", e))?;
        }
        Ok(())
    }
}