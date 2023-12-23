use crate::device::Device;
use crate::DeviceControllerWrapper;
use crate::storage::Storage;

#[tauri::command]
pub fn get_devices(storage: tauri::State<Storage>,
                   device_controller_wrapper: tauri::State<DeviceControllerWrapper>) -> Vec<Device> {
    device_controller_wrapper.controller.lock().unwrap().refresh_devices();
    storage.get_devices()
}

#[tauri::command]
pub fn set_state(device_controller_wrapper: tauri::State<DeviceControllerWrapper>,
                 device_ip: String,
                 state: bool) -> bool {
    match device_controller_wrapper.controller.lock() {
        Ok(controller) => {
            controller.set_state(device_ip, state);
            true
        }
        Err(_) => false,
    }
}

#[tauri::command]
pub fn set_pilot(device_controller_wrapper: tauri::State<DeviceControllerWrapper>,
                 device_ip: String,
                 params: serde_json::Value) -> bool {
    match device_controller_wrapper.controller.lock() {
        Ok(controller) => {
            controller.set_pilot(device_ip, params);
            true
        }
        Err(_) => false,
    }
}