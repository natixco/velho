use std::sync::Arc;

use tauri::{AppHandle, Config, Env, PackageInfo};
use tauri::api::path::{BaseDirectory, resolve_path};

use crate::light::Light;

static STORAGE_FILE_NAME: &str = "lights.json";

pub struct AppInfo {
    pub handle: AppHandle,
    pub config: Arc<Config>,
    pub package_info: PackageInfo,
    pub env: Env,
}

pub struct Storage {
    app: AppInfo,
}

impl Storage {
    pub fn new(app: AppInfo) -> std::io::Result<Self> {
        Ok(Self {
            app,
        })
    }

    pub fn load(&self) -> Vec<Light> {
        let path = self.get_storage_path();
        let json = std::fs::read_to_string(path);
        if let Ok(json) = json {
            let lights: Vec<Light> = serde_json::from_str(&json).unwrap();
            return lights;
        } else {
            println!("Error while loading storage: {}", json.unwrap_err());
        }

        Vec::new()
    }

    pub fn save(&self, lights: Vec<Light>) -> () {
        self.ensure_storage_file();

        let json = serde_json::to_string(&lights).unwrap();
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