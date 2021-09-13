use crate::room::Room;
use std::fs;
use walkdir::WalkDir;

pub fn load_room_templates(path: &str) -> Option<Vec<Room<i32>>> {
    let mut templates: Vec<String> = Vec::new();
    for dir_entry in WalkDir::new(path) {
        if let Ok(entry) = dir_entry {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                templates.push(content);
            }
        }
    }

    if templates.is_empty() {
        None
    } else {
        Some(
            templates
                .iter()
                .map(|template_string| Room::<i32>::from_template_string(template_string.clone()))
                .collect(),
        )
    }
}
