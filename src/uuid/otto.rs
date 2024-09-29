const UUID_NILL: &str = "00000000-0000-0000-0000-000000000000";
const UUID_MAX: &str = "FFFFFFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF";

pub struct Manager {
    nil: String,
    max: String,
    bytes: String,
}

impl Default for Manager {
    fn default() -> Self {
        Self {
            nil: String::from(UUID_NILL),
            max: String::from(UUID_MAX),
            bytes: String::new(),
        }
    }
}

impl Manager {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn get_bytes(self) -> String {
        self.bytes
    }
    pub fn get_nill(self) -> String {
        self.nil
    }
    pub fn get_max(self) -> String {
        self.max
    }
}
