use crate::room::Room;
use std::fs;
use walkdir::WalkDir;

#[derive(Clone)]
pub struct RoomPaths {
    pub name: String,
    pub base_template_path: String,
    pub fill_template_paths: Vec<String>,
}

impl RoomPaths {
    pub fn load_rooms(&self) -> Option<Vec<Room>> {
        let mut templates: Vec<Vec<String>> = Vec::new();

        for entry in WalkDir::new(self.base_template_path.as_str())
            .into_iter()
            .flatten()
        {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                let mut room_templates: Vec<Vec<String>> = Vec::new();

                if self.fill_template_paths.is_empty() {
                    room_templates.push(vec![content.clone()])
                }

                for fill_path in &self.fill_template_paths {
                    let mut room_template: Vec<String> = vec![content.clone()];
                    for fill_entry in WalkDir::new(fill_path).into_iter().flatten() {
                        if let Ok(item_content) = fs::read_to_string(fill_entry.path()) {
                            room_template.push(item_content.clone());
                        }
                    }
                    room_templates.push(room_template);
                }

                room_templates
                    .iter()
                    .for_each(|template| templates.push(template.clone()));
            }
        }

        if templates.is_empty() {
            None
        } else {
            Some(
                templates
                    .iter()
                    .map(|template_string| Room::from_template_strings(template_string.clone()))
                    .collect(),
            )
        }
    }
}
