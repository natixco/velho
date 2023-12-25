use std::sync::{Arc, Mutex};

use serde_json::Value;
use tauri::{AppHandle, Config, Env, Manager, PackageInfo};
use tauri::api::path::{BaseDirectory, resolve_path};

use crate::light::Light;

static STORAGE_FILE_NAME: &str = "data.json";

pub struct AppInfo {
    pub handle: AppHandle,
    pub config: Arc<Config>,
    pub package_info: PackageInfo,
    pub env: Env,
}

pub struct Storage {
    app: AppInfo,
    lights: Mutex<Vec<Light>>,
}

impl Storage {
    pub fn new(app: AppInfo) -> std::io::Result<Self> {
        Ok(Self {
            app,
            lights: Mutex::new(Vec::new()),
        })
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

        self.app.handle.get_window("main").unwrap().emit("upsert_light", _light.clone()).unwrap();

        drop(lights);
        self.save();
    }

    pub fn update_light(&self, mac: String, params: Value) -> bool {
        let mut lights = self.lights.lock().unwrap();

        if let Some(light) = lights.iter_mut().find(|d| d.state.mac == mac) {
            if let Some(name) = params.get("name") {
                light.name = name
                    .as_str()
                    .map_or_else(|| light.state.mac.clone(), |s| s.to_string());
            }

            return true;
        }

        drop(lights);
        self.save();

        false
    }

    pub fn load(&mut self) -> () {
        let path = self.get_storage_path();
        let json = std::fs::read_to_string(path);
        if let Ok(json) = json {
            let mut lights: Vec<Light> = serde_json::from_str(&json).unwrap();
            self.lights.lock().unwrap().append(&mut lights);
        }
    }

    fn save(&self) -> () {
        self.ensure_storage_file();

        let json = serde_json::to_string(&self.lights.lock().unwrap().to_vec()).unwrap();
        let path = self.get_storage_path();
        if let Err(error) = std::fs::write(path, json) {
            println!("Error while saving storage: {}", error);
        }
    }

    fn ensure_storage_file(&self) -> () {
        let path = self.get_storage_path();
        let parent = path.parent().unwrap();

        if !parent.exists() {
            if let Err(error) = std::fs::create_dir(parent) {
                println!("Error while creating storage dir: {}", error);
            };
        }
    }

    fn get_storage_path(&self) -> std::path::PathBuf {
        resolve_path(&self.app.config, &self.app.package_info, &self.app.env, STORAGE_FILE_NAME, Some(BaseDirectory::AppData)).unwrap()
    }
}