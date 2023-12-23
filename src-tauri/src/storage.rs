use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Config, Env, Manager, PackageInfo};
use tauri::api::path::{BaseDirectory, resolve_path};

use crate::device::Device;

static STORAGE_FILE_NAME: &str = "data.json";

pub struct AppInfo {
    pub handle: AppHandle,
    pub config: Arc<Config>,
    pub package_info: PackageInfo,
    pub env: Env,
}

pub struct Storage {
    app: AppInfo,
    devices: Mutex<Vec<Device>>,
}

impl Storage {
    pub fn new(app: AppInfo) -> std::io::Result<Self> {
        Ok(Self {
            app,
            devices: Mutex::new(Vec::new()),
        })
    }

    pub fn get_devices(&self) -> Vec<Device> {
        let devices = self.devices.lock().unwrap();
        devices.to_vec()
    }

    pub fn upsert_device(&self, device: Device) -> () {
        let device_clone = device.clone();
        let mut devices = self.devices.lock().unwrap();
        match devices.iter().position(|d| d.mac == device.mac) {
            Some(index) => devices[index] = device,
            None => devices.push(device),
        }
        self.app.handle.get_window("main").unwrap().emit("device_discovery", device_clone).unwrap();
        self.save(devices.to_vec());
    }

    pub fn load(&mut self) -> () {
        let path = self.get_storage_path();
        let json = std::fs::read_to_string(path);
        match json {
            Ok(json) => {
                let mut devices: Vec<Device> = serde_json::from_str(&json).unwrap();
                self.devices.lock().unwrap().append(&mut devices);
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }

    fn save(&self, devices: Vec<Device>) -> () {
        self.ensure_storage_file();

        let json = serde_json::to_string(&devices).unwrap();
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