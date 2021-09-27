use crate::room::Room;
use std::fs;
use walkdir::WalkDir;

pub struct RoomPaths {
    pub name: &'static str,
    pub base_template_path: &'static str,
    pub fill_template_path: &'static str,
}

impl RoomPaths {
    pub fn load_rooms(&self) -> Option<Vec<Room<i32>>> {
        let mut templates: Vec<String> = Vec::new();

        for entry in WalkDir::new(self.base_template_path).into_iter().flatten() {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                templates.push(content);
            }
        }

        if templates.is_empty() {
            None
        } else {
            Some(
                templates
                    .iter()
                    .map(|template_string| {
                        Room::<i32>::from_template_string(template_string.clone())
                    })
                    .collect(),
            )
        }
    }
}
