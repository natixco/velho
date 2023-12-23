use tauri::{AppHandle, Manager};
use crate::device::Device;
use crate::storage::Storage;

#[tauri::command]
pub fn get_devices(app_handle: AppHandle) -> Vec<Device> {
    let storage = app_handle.state::<Storage>();
    storage.get_devices()
}