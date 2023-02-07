use serde::{Deserialize, Serialize};

use super::{cells::Cell, layers::LayerType};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Room {
    cells: Vec<Cell>,
    max_side_length: u16,
}

impl From<&String> for Room {
    fn from(value: &String) -> Self {
        let mut max_side_length: u16 = 0;
        let mut cells: Vec<Cell> = Vec::new();
        let mut lines: Vec<&str> = value.split('\n').collect();
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
                cells.push(Self::build_cell(x as i32, y as i32, c));
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

        Room {
            cells,
            max_side_length,
        }
    }
}

impl From<&str> for Room {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<String> for Room {
    fn from(value: String) -> Self {
        Self::from(&value)
    }
}

impl Room {
    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn max_side_length(&self) -> u16 {
        self.max_side_length
    }

    pub fn translate(&self, bottom_left_x: i32, bottom_left_y: i32) -> Self {
        let cells = self
            .cells
            .iter()
            .map(|cell| {
                cell.translate(
                    cell.coordinate().x() + bottom_left_x,
                    cell.coordinate().y() + bottom_left_y,
                )
            })
            .collect();

        Self {
            cells,
            max_side_length: self.max_side_length,
        }
    }

    pub fn spawnable_cells(&self) -> Vec<&Cell> {
        self.cells
            .iter()
            .filter(|cell| cell.visible_layer().eq(&LayerType::RoomFloor))
            .collect()
    }

    pub fn possible_door_cells(&self) -> Vec<&Cell> {
        self.cells
            .iter()
            .filter(|cell| self.cell_can_be_door(cell))
            .collect()
    }

    pub fn cell_can_be_door(&self, cell: &Cell) -> bool {
        if cell.visible_layer().ne(&LayerType::RoomWall) {
            return false;
        }

        let surrounding = vec![
            (cell.coordinate().x() - 1, cell.coordinate().y()),
            (cell.coordinate().x(), cell.coordinate().y() - 1),
            (cell.coordinate().x() + 1, cell.coordinate().y()),
            (cell.coordinate().x(), cell.coordinate().y() + 1),
        ];

        let next_to_floor = surrounding.iter().any(|(x, y)| {
            self.cell_at(*x, *y)
                .map(|other_cell| other_cell.visible_layer().eq(&LayerType::RoomFloor))
                .unwrap_or_default()
        });
        let next_to_exterior = surrounding
            .iter()
            .any(|(x, y)| self.cell_at(*x, *y).is_none());

        next_to_floor && next_to_exterior
    }

    pub fn cell_at(&self, x: i32, y: i32) -> Option<&Cell> {
        self.cells.iter().find(|cell| cell.is_at_location(x, y))
    }

    pub fn replace_cell_contents(&mut self, x: i32, y: i32, layer: LayerType) {
        if let Some(cell) = self.cells.iter_mut().find(|cell| cell.is_at_location(x, y)) {
            cell.clear_contents();
            cell.add_layer(layer);
        }
    }

    pub fn add_layer_to_cell(&mut self, x: i32, y: i32, layer: LayerType) {
        if let Some(cell) = self.cells.iter_mut().find(|cell| cell.is_at_location(x, y)) {
            cell.add_layer(layer);
        }
    }

    fn build_cell(x: i32, y: i32, c: char) -> Cell {
        let layer: LayerType = c.into();
        let mut cell = Cell::from((x, y));
        if layer.ne(&LayerType::RoomFloor) && layer.ne(&LayerType::Door) {
            cell.add_layer(LayerType::RoomFloor);
        }
        cell.add_layer(layer);
        cell
    }
}

#[cfg(test)]
mod tests {
    use crate::map::{
        cells::{Cell, Coordinate},
        layers::LayerType,
    };

    use super::Room;
    use std::fs;

    #[test]
    fn translate_moves_room() {
        let room = Room {
            cells: vec![Cell::from((Coordinate::from(1), LayerType::RoomFloor))],
            max_side_length: 4,
        };

        let translated = room.translate(4, 6);
        let translated_cell = translated.cells.first().unwrap();

        assert_eq!(translated_cell.coordinate().x(), 5);
        assert_eq!(translated_cell.coordinate().y(), 7);
    }

    #[test]
    fn from_template_string_builds_right_side_length() {
        let template_string = "wwww\nwffw\nwffd\nwwww".to_string();
        let room = Room::from(template_string);
        assert_eq!(room.max_side_length, 4);
    }

    #[test]
    fn from_template_string_builds_right_room() {
        // let template_string = "wwww\nwffw\nwffd\nwwww".to_string();
        let template = "
        ||||
        |==|
        |==D
        ||||
        "
        .to_string();
        let room = Room::from(template);
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
        let room = Room::from(template);
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
