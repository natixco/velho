use std::sync::{Arc, Mutex};

use crate::light_controller::LightController;

pub struct LightControllerWrapper {
    pub controller: Arc<Mutex<LightController>>,
}

impl LightControllerWrapper {
    pub fn new(controller: LightController) -> Self {
        Self {
            controller: Arc::new(Mutex::new(controller)),
        }
    }
}