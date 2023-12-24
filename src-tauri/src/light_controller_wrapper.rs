use std::sync::{Arc, Mutex};

use crate::light_controller::LightController;

pub struct LightControllerWrapper {
    pub controller: Arc<Mutex<LightController>>,
}