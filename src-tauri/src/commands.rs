use crate::light::Light;
use crate::light_controller_wrapper::LightControllerWrapper;
use crate::light_storage::LightStorage;

#[tauri::command]
pub fn get_lights(
    storage: tauri::State<LightStorage>,
    light_controller_wrapper: tauri::State<LightControllerWrapper>,
) -> Result<Vec<Light>, String> {
    light_controller_wrapper.controller
        .lock()
        .map_err(|e| format!("Failed to acquire controller lock: {}", e))?
        .refresh_devices()
        .map(|_| true)
        .map_err(|e| format!("Failed to refresh devices: {}", e))?;

    storage.get_lights()
}

#[tauri::command]
pub fn set_pilot(
    light_controller_wrapper: tauri::State<LightControllerWrapper>,
    ip: String,
    params: serde_json::Value,
) -> Result<bool, String> {
    light_controller_wrapper.controller
        .lock()
        .map_err(|e| format!("Failed to acquire controller lock: {}", e))?
        .set_pilot(ip, params)
        .map(|_| true)
        .map_err(|e| format!("Failed to set pilot: {}", e))
}

#[tauri::command]
pub fn update_light(
    storage: tauri::State<LightStorage>,
    mac: String,
    params: serde_json::Value,
) -> Result<bool, String> {
    storage.update_light(mac, params)
}