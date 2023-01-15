#[derive(Debug)]
pub struct GenerationError {
    pub message: String,
}

impl GenerationError {
    pub fn no_room_paths() -> Self {
        Self {
            message: "no_room_paths".to_string(),
        }
    }

    pub fn room_templates_cannot_be_loaded() -> Self {
        Self {
            message: "room_templates_cannot_be_loaded".to_string(),
        }
    }
}
