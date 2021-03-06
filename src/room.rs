use crate::{cells::cell::Cell, cells::layer::LayerType};

#[derive(Clone)]
pub struct Room {
    pub cells: Vec<Cell>,
    pub max_side_length: usize,
}

impl Room {
    pub fn translate(&self, bottom_left_x: i32, bottom_left_y: i32) -> Self {
        let cells = self
            .cells
            .iter()
            .map(|cell| {
                cell.translate(
                    cell.coordinate.x + bottom_left_x,
                    cell.coordinate.y + bottom_left_y,
                )
            })
            .collect();

        Self {
            cells,
            max_side_length: self.max_side_length,
        }
    }

    pub fn cell_at(&self, x: i32, y: i32) -> Option<&Cell> {
        return self.cells.iter().find(|cell| cell.is_at_location(x, y));
    }

    pub fn from_template_strings(templates: Vec<String>) -> Self {
        let mut max_side_length: usize = 0;
        let mut cells: Vec<Cell> = Vec::new();

        for template in templates {
            let mut lines: Vec<&str> = template.split('\n').collect();
            // Need to reverse so that we get the bottom ones as the first
            // cells that we create.
            lines.reverse();

            let mut y_side_length = 0;

            for (y, line) in lines
                .iter()
                .filter(|line| !line.trim().is_empty())
                .enumerate()
            {
                let mut side_length = 0;
                if line.chars().filter(|c| !c.is_whitespace()).count() == 0 {
                    continue;
                }

                let trimmed = line.trim();

                for (x, c) in trimmed.chars().enumerate() {
                    if let Some(existing_cell) = cells
                        .iter_mut()
                        .find(|c| c.is_at_location(x as i32, y as i32))
                    {
                        existing_cell.add_layer(&LayerType::from(c));
                    } else {
                        cells.push(Self::build_cell(x as i32, y as i32, c));
                    }
                    side_length += 1;
                }

                if side_length > max_side_length {
                    max_side_length = side_length;
                }

                y_side_length += 1;
            }

            if y_side_length > max_side_length {
                max_side_length = y_side_length;
            }
        }

        Room {
            cells,
            max_side_length,
        }
    }

    /// Creates a Room from a string that is structured like
    ///
    /// wwww
    /// wffw
    /// wffd
    /// wwww
    ///
    /// Where w is wall, f is floor, d is door (as an example)
    pub fn from_template_string(template: String) -> Self {
        Self::from_template_strings(vec![template])
    }

    fn build_cell(x: i32, y: i32, c: char) -> Cell {
        let layer = LayerType::from(c);
        Cell::new(x, y, layer)
    }
}

#[cfg(test)]
mod tests {
    use crate::{cells::cell::Cell, cells::layer::LayerType, room::Room};
    use std::fs;

    #[test]
    fn translate_moves_room() {
        let room = Room {
            cells: vec![Cell::splatted_room_floor(1)],
            max_side_length: 4,
        };

        let translated = room.translate(4, 6);
        let translated_cell = translated.cells.first().unwrap();

        assert_eq!(translated_cell.coordinate.x, 5);
        assert_eq!(translated_cell.coordinate.y, 7);
    }

    #[test]
    fn from_template_string_builds_right_side_length() {
        let template_string = "wwww\nwffw\nwffd\nwwww".to_string();
        let room = Room::from_template_string(template_string);
        assert_eq!(room.max_side_length, 4);
    }

    #[test]
    fn from_template_strings_builds_right_room() {
        let base_template = "
        wwww
        wffw
        wffd
        wwww
        "
        .to_string();
        let rubble = "
        eeee
        eree
        eeee
        eeee
        "
        .to_string();
        let templates = vec![base_template, rubble];
        let room = Room::from_template_strings(templates);
        let cell_1_2 = room.cell_at(1, 2).unwrap();
        let cell_1_1 = room.cell_at(1, 1).unwrap();
        assert_eq!(cell_1_1.layers.len(), 1);
        assert_eq!(
            cell_1_1.cell_type_at_layer(0).unwrap(),
            LayerType::RoomFloor
        );

        assert_eq!(cell_1_2.layers.len(), 2);
        assert_eq!(cell_1_2.cell_type_at_layer(1).unwrap(), LayerType::Rubble);
    }

    #[test]
    fn from_template_string_builds_right_room() {
        // let template_string = "wwww\nwffw\nwffd\nwwww".to_string();
        let template = "
        wwww
        wffw
        wffd
        wwww
        "
        .to_string();
        let room = Room::from_template_string(template);
        assert_eq!(room.cells.len(), 16);

        // Bottom wall
        let cell_0_0 = room.cell_at(0, 0).unwrap();
        assert_eq!(cell_0_0.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_1_0 = room.cell_at(1, 0).unwrap();
        assert_eq!(cell_1_0.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_2_0 = room.cell_at(2, 0).unwrap();
        assert_eq!(cell_2_0.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_3_0 = room.cell_at(3, 0).unwrap();
        assert_eq!(cell_3_0.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);

        // Lower room
        let cell_0_1 = room.cell_at(0, 1).unwrap();
        assert_eq!(cell_0_1.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_1_1 = room.cell_at(1, 1).unwrap();
        assert_eq!(cell_1_1.cell_type(), LayerType::RoomFloor);
        let cell_2_1 = room.cell_at(2, 1).unwrap();
        assert_eq!(cell_2_1.cell_type(), LayerType::RoomFloor);
        let cell_3_1 = room.cell_at(3, 1).unwrap();
        assert_eq!(cell_3_1.cell_type(), LayerType::Door);

        // Upper room
        let cell_0_2 = room.cell_at(0, 2).unwrap();
        assert_eq!(cell_0_2.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_1_2 = room.cell_at(1, 2).unwrap();
        assert_eq!(cell_1_2.cell_type(), LayerType::RoomFloor);
        let cell_2_2 = room.cell_at(2, 2).unwrap();
        assert_eq!(cell_2_2.cell_type(), LayerType::RoomFloor);
        let cell_3_2 = room.cell_at(3, 2).unwrap();
        assert_eq!(cell_3_2.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);

        // Top wall
        let cell_0_3 = room.cell_at(0, 3).unwrap();
        assert_eq!(cell_0_3.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_1_3 = room.cell_at(1, 3).unwrap();
        assert_eq!(cell_1_3.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_2_3 = room.cell_at(2, 3).unwrap();
        assert_eq!(cell_2_3.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_3_3 = room.cell_at(3, 3).unwrap();
        assert_eq!(cell_3_3.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
    }

    #[test]
    fn builds_room_from_funky_text() {
        let template = fs::read_to_string("assets/builds_room_from_funky_text.txt").unwrap();
        let room = Room::from_template_string(template);
        assert_eq!(room.max_side_length, 7);
        assert_eq!(room.cells.len(), 36);

        let cell_0_0 = room.cell_at(0, 0).unwrap();
        assert_eq!(cell_0_0.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_1_0 = room.cell_at(1, 0).unwrap();
        assert_eq!(cell_1_0.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_2_0 = room.cell_at(2, 0).unwrap();
        assert_eq!(cell_2_0.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);

        let cell_0_1 = room.cell_at(0, 1).unwrap();
        assert_eq!(cell_0_1.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_1_1 = room.cell_at(1, 1).unwrap();
        assert_eq!(cell_1_1.cell_type(), LayerType::RoomFloor);
        let cell_2_1 = room.cell_at(2, 1).unwrap();
        assert_eq!(cell_2_1.cell_type(), LayerType::RoomFloor);
        let cell_3_1 = room.cell_at(3, 1).unwrap();
        assert_eq!(cell_3_1.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);

        let cell_0_2 = room.cell_at(0, 2).unwrap();
        assert_eq!(cell_0_2.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_1_2 = room.cell_at(1, 2).unwrap();
        assert_eq!(cell_1_2.cell_type(), LayerType::RoomFloor);
        let cell_2_2 = room.cell_at(2, 2).unwrap();
        assert_eq!(cell_2_2.cell_type(), LayerType::RoomFloor);
        let cell_3_2 = room.cell_at(3, 2).unwrap();
        assert_eq!(cell_3_2.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_4_2 = room.cell_at(3, 2).unwrap();
        assert_eq!(cell_4_2.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);

        let cell_0_3 = room.cell_at(0, 3).unwrap();
        assert_eq!(cell_0_3.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_1_3 = room.cell_at(1, 3).unwrap();
        assert_eq!(cell_1_3.cell_type(), LayerType::RoomFloor);
        let cell_2_3 = room.cell_at(2, 3).unwrap();
        assert_eq!(cell_2_3.cell_type(), LayerType::RoomFloor);
        let cell_3_3 = room.cell_at(3, 3).unwrap();
        assert_eq!(cell_3_3.cell_type(), LayerType::RoomFloor);
        let cell_4_3 = room.cell_at(4, 3).unwrap();
        assert_eq!(cell_4_3.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_5_3 = room.cell_at(5, 3).unwrap();
        assert_eq!(cell_5_3.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);

        let cell_0_4 = room.cell_at(0, 4).unwrap();
        assert_eq!(cell_0_4.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_1_4 = room.cell_at(1, 4).unwrap();
        assert_eq!(cell_1_4.cell_type(), LayerType::RoomFloor);
        let cell_2_4 = room.cell_at(2, 4).unwrap();
        assert_eq!(cell_2_4.cell_type(), LayerType::RoomFloor);
        let cell_3_4 = room.cell_at(3, 4).unwrap();
        assert_eq!(cell_3_4.cell_type(), LayerType::RoomFloor);
        let cell_4_4 = room.cell_at(4, 4).unwrap();
        assert_eq!(cell_4_4.cell_type(), LayerType::RoomFloor);
        let cell_5_4 = room.cell_at(5, 4).unwrap();
        assert_eq!(cell_5_4.cell_type(), LayerType::Door);

        let cell_0_5 = room.cell_at(0, 5).unwrap();
        assert_eq!(cell_0_5.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_1_5 = room.cell_at(1, 5).unwrap();
        assert_eq!(cell_1_5.cell_type(), LayerType::RoomFloor);
        let cell_2_5 = room.cell_at(2, 5).unwrap();
        assert_eq!(cell_2_5.cell_type(), LayerType::RoomFloor);
        let cell_3_5 = room.cell_at(3, 5).unwrap();
        assert_eq!(cell_3_5.cell_type(), LayerType::RoomFloor);
        let cell_4_5 = room.cell_at(4, 5).unwrap();
        assert_eq!(cell_4_5.cell_type(), LayerType::RoomFloor);
        let cell_5_5 = room.cell_at(5, 5).unwrap();
        assert_eq!(cell_5_5.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);

        let cell_0_6 = room.cell_at(0, 6).unwrap();
        assert_eq!(cell_0_6.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_1_6 = room.cell_at(1, 6).unwrap();
        assert_eq!(cell_1_6.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_2_6 = room.cell_at(2, 6).unwrap();
        assert_eq!(cell_2_6.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_3_6 = room.cell_at(3, 6).unwrap();
        assert_eq!(cell_3_6.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_4_6 = room.cell_at(4, 6).unwrap();
        assert_eq!(cell_4_6.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
        let cell_5_6 = room.cell_at(5, 6).unwrap();
        assert_eq!(cell_5_6.cell_type_at_layer(1).unwrap(), LayerType::RoomWall);
    }
}
