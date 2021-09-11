use crate::cell::Cell;

#[derive(Clone)]
pub struct Room<T: Copy + std::ops::Add<Output = T>> {
    pub cells: Vec<Cell<T>>,
    pub max_side_length: usize,
}

impl<T: Copy + std::ops::Add<Output = T>> Room<T> {
    pub fn translate(&self, bottom_left_x: T, bottom_left_y: T) -> Self {
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

        return Self {
            cells,
            max_side_length: self.max_side_length,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::{cell::Cell, room::Room};

    #[test]
    fn translate_moves_room() {
        let room = Room::<i32> {
            cells: vec![
                Cell::<i32>::splatted_room_floor(1),
            ],
            max_side_length: 4,
        };

        let translated = room.translate(4, 6);
        let translated_cell = translated.cells.first().unwrap();

        assert_eq!(translated_cell.coordinate.x, 5);
        assert_eq!(translated_cell.coordinate.y, 7);
    }
}
