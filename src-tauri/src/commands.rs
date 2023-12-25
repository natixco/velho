use crate::light::Light;
use crate::LightControllerWrapper;
use crate::storage::Storage;

#[tauri::command]
pub fn get_lights(storage: tauri::State<Storage>,
                  light_controller_wrapper: tauri::State<LightControllerWrapper>) -> Vec<Light> {
    if let Ok(controller) = light_controller_wrapper.controller.lock() {
        controller.refresh_devices();
    }
    storage.get_lights()
}

#[tauri::command]
pub fn set_pilot(light_controller_wrapper: tauri::State<LightControllerWrapper>,
                 ip: String,
                 params: serde_json::Value) -> bool {
    if let Ok(controller) = light_controller_wrapper.controller.lock() {
        controller.set_pilot(ip, params);
        return true;
    }

    false
}

#[tauri::command]
pub fn update_light(storage: tauri::State<Storage>,
                    mac: String,
                    params: serde_json::Value) -> bool {
    storage.update_light(mac, params)
}