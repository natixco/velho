use std::path::PathBuf;
use std::sync::Arc;

use tauri::api::path::{resolve_path, BaseDirectory};
use tauri::{AppHandle, Config, Env, PackageInfo};

use crate::constants::STORAGE_FILE_NAME;
use crate::light::Light;

#[derive(Clone)]
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
        Ok(Self { app })
    }

    pub fn load(&self) -> Result<Vec<Light>, String> {
        let path = self.get_storage_path()?;

        match std::fs::read_to_string(&path) {
            Ok(json) => {
                serde_json::from_str::<Vec<Light>>(&json)
                    .map_err(|e| format!("Failed to parse storage file: {}", e))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                Ok(Vec::new())
            }
            Err(e) => {
                Err(format!("Failed to read storage file: {}", e))
            }
        }
    }

    pub fn save(&self, lights: &[Light]) -> Result<(), String> {
        self.ensure_storage_directory()?;

        let json = serde_json::to_string_pretty(lights)
            .map_err(|e| format!("Failed to serialize lights: {}", e))?;

        let path = self.get_storage_path()?;

        std::fs::write(&path, json)
            .map_err(|e| format!("Failed to write storage file: {}", e))
    }

    fn ensure_storage_directory(&self) -> Result<(), String> {
        let path = self.get_storage_path()?;

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create storage directory: {}", e))?;
            }
        }

        Ok(())
    }

    fn get_storage_path(&self) -> Result<PathBuf, String> {
        resolve_path(
            &self.app.config,
            &self.app.package_info,
            &self.app.env,
            STORAGE_FILE_NAME,
            Some(BaseDirectory::AppData),
        )
            .map_err(|e| format!("Failed to resolve storage path: {}", e))
    }
}