use std::fs;

use walkdir::WalkDir;

use crate::map::Room;

#[derive(Clone)]
pub struct RoomPaths {
    pub name: String,
    pub template_path: String,
}

impl RoomPaths {
    pub fn load_rooms(&self) -> Option<Vec<Room>> {
        let mut templates: Vec<Room> = Vec::new();

        for entry in WalkDir::new(self.template_path.as_str())
            .into_iter()
            .flatten()
        {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                templates.push(Room::from(&content));
            }
        }

        if templates.is_empty() {
            None
        } else {
            Some(templates)
        }
    }
}
