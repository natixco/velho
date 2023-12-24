use std::sync::{Arc, Mutex};

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

    pub fn upsert_light(&self, light: Light) -> () {
        let device_clone = light.clone();
        let mut lights = self.lights.lock().unwrap();
        match lights.iter().position(|d| d.state.mac == light.state.mac) {
            Some(index) => lights[index] = light,
            None => lights.push(light),
        }
        self.app.handle.get_window("main").unwrap().emit("upsert_light", device_clone).unwrap();
        self.save(lights.to_vec());
    }

    pub fn load(&mut self) -> () {
        let path = self.get_storage_path();
        let json = std::fs::read_to_string(path);
        match json {
            Ok(json) => {
                let mut lights: Vec<Light> = serde_json::from_str(&json).unwrap();
                self.lights.lock().unwrap().append(&mut lights);
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }

    fn save(&self, lights: Vec<Light>) -> () {
        self.ensure_storage_file();

        let json = serde_json::to_string(&lights).unwrap();
        let path = self.get_storage_path();
        if let Err(error) = std::fs::write(path, json) {
            println!("Error: {}", error);
        }
    }

    fn ensure_storage_file(&self) -> () {
        let path = self.get_storage_path();
        let parent = path.parent().unwrap();

        if !parent.exists() {
            if let Err(error) = std::fs::create_dir(parent) {
                println!("Error: {}", error);
            };
        }
    }

    fn get_storage_path(&self) -> std::path::PathBuf {
        resolve_path(&self.app.config, &self.app.package_info, &self.app.env, STORAGE_FILE_NAME, Some(BaseDirectory::AppData)).unwrap()
    }
}