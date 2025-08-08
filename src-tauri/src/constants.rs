/// Network configuration constants
pub const LOCAL_ADDRESS: &str = "0.0.0.0";
pub const BROADCAST_ADDRESS: &str = "255.255.255.255";
pub const PORT: u16 = 38899;
pub const BUFFER_SIZE: usize = 1024;

/// Storage configuration constants
pub const STORAGE_FILE_NAME: &str = "lights.json";

/// Window configuration constants
pub const MAIN_WINDOW_NAME: &str = "main";

/// Event names
pub const UPSERT_LIGHT_EVENT: &str = "upsert_light";

/// JSON field names
pub const MAC_FIELD: &str = "mac";
pub const SCENE_ID_FIELD: &str = "sceneId";
pub const DIMMING_FIELD: &str = "dimming";
pub const STATE_FIELD: &str = "state";
pub const TEMP_FIELD: &str = "temp";
pub const NAME_FIELD: &str = "name";
pub const METHOD_FIELD: &str = "method";
pub const PARAMS_FIELD: &str = "params";
pub const RESULT_FIELD: &str = "result";
pub const SUCCESS_FIELD: &str = "success";

/// Method names
pub const GET_PILOT_METHOD: &str = "getPilot";
pub const SET_PILOT_METHOD: &str = "setPilot"; 