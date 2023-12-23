use std::sync::{Arc, Mutex};

use crate::device_controller::DeviceController;

pub struct DeviceControllerWrapper {
    pub controller: Arc<Mutex<DeviceController>>,
}