use crate::{cells::layer::LayerType, coordinate::Coordinate};

#[derive(PartialEq, Eq, Clone)]
pub struct Cell {
    pub coordinate: Coordinate,
    pub layers: Vec<LayerType>,
}

impl Cell {
    pub fn clear_contents(&mut self) {
        self.layers.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.cell_type() == LayerType::Empty
    }

    pub fn set_to_floor(&mut self) {
        self.layers.clear();
        self.layers.push(LayerType::Floor);
    }

    pub fn is_at_location(&self, x: i32, y: i32) -> bool {
        self.coordinate.x == x && self.coordinate.y == y
    }

    pub fn cell_type(&self) -> LayerType {
        if self.layers.is_empty() {
            return LayerType::Empty;
        }

        self.cell_type_at_layer(0).unwrap()
    }

    pub fn cell_type_at_layer(&self, layer_index: usize) -> Option<LayerType> {
        if let Some(cell_type) = self.layers.get(layer_index) {
            return Some(*cell_type);
        }

        None
    }

    pub fn add_layer(&mut self, layer: &LayerType) {
        // We don't need empty cells
        if layer != &LayerType::Empty {
            self.layers.push(*layer);
        }
    }

    pub fn add_cell_layers(&mut self, layers: &[LayerType]) {
        for layer in layers {
            self.layers.push(*layer)
        }
    }

    pub fn set_cell_type(&mut self, cell_type: LayerType) {
        if !self.layers.is_empty() {
            self.layers.clear()
        }
        self.layers.push(cell_type);
    }

    pub fn is_walkable(&self) -> bool {
        if self.layers.is_empty() {
            false
        } else {
            self.layers.iter().all(|layer| layer.is_walkable())
        }
    }

    pub fn is_spawnable(&self) -> bool {
        if self.layers.is_empty() {
            false
        } else {
            self.layers.iter().all(|layer| layer.is_spawnable())
        }
    }

    pub fn translate(&self, x: i32, y: i32) -> Self {
        Self {
            coordinate: Coordinate::new(x, y),
            layers: self.layers.clone(),
        }
    }

    pub fn new(x: i32, y: i32, cell_type: LayerType) -> Self {
        let layers: Vec<LayerType> = if cell_type != LayerType::Empty {
            vec![cell_type]
        } else {
            Vec::new()
        };

        Self {
            layers,
            coordinate: Coordinate::new(x, y),
        }
    }

    pub fn splat(value: i32, cell_type: LayerType) -> Self {
        Self {
            coordinate: Coordinate::splat(value),
            layers: vec![cell_type],
        }
    }

    pub fn splatted_room_wall(value: i32) -> Self {
        Self::splat(value, LayerType::RoomWall)
    }

    pub fn room_wall(x: i32, y: i32) -> Self {
        Self::new(x, y, LayerType::RoomWall)
    }

    pub fn splatted_room_floor(value: i32) -> Self {
        Self::splat(value, LayerType::RoomFloor)
    }

    pub fn room_floor(x: i32, y: i32) -> Self {
        Self::new(x, y, LayerType::RoomFloor)
    }

    pub fn room_door(x: i32, y: i32) -> Self {
        Self::new(x, y, LayerType::Door)
    }

    pub fn outer_wall(x: i32, y: i32) -> Self {
        Self::new(x, y, LayerType::OuterWall)
    }

    pub fn empty_cell(x: i32, y: i32) -> Self {
        Self::new(x, y, LayerType::Empty)
    }
}

#[cfg(test)]
mod tests {
    mod set_cell_type {
        use crate::{cells::cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_changes_the_cell_type() {
            let mut cell = Cell {
                coordinate: Coordinate::splat(2),
                layers: vec![],
            };
            cell.set_cell_type(crate::cells::layer::LayerType::RoomFloor);
            assert_eq!(cell.cell_type(), crate::cells::layer::LayerType::RoomFloor);
        }

        #[test]
        fn it_updates_spawnable_walkable_for_room_floor() {
            let mut cell = Cell {
                coordinate: Coordinate::splat(2),
                layers: vec![],
            };
            cell.set_cell_type(crate::cells::layer::LayerType::RoomFloor);
            assert!(cell.is_spawnable());
            assert!(cell.is_walkable());
        }

        #[test]
        fn it_updates_spawnable_walkable_for_floor() {
            let mut cell = Cell {
                coordinate: Coordinate::splat(2),
                layers: vec![],
            };
            cell.set_cell_type(crate::cells::layer::LayerType::Floor);
            assert!(cell.is_spawnable());
            assert!(cell.is_walkable());
        }

        #[test]
        fn it_updates_spawnable_walkable_for_door() {
            let mut cell = Cell {
                coordinate: Coordinate::splat(2),
                layers: vec![],
            };
            cell.set_cell_type(crate::cells::layer::LayerType::Door);
            assert!(!cell.is_spawnable());
            assert!(cell.is_walkable());
        }
    }

    mod translate {
        use crate::{cells::cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_updates_cell_coordinates() {
            let cell = Cell {
                coordinate: Coordinate::splat(2),
                layers: vec![],
            };
            let new_cell = cell.translate(3, 3);

            assert_eq!(new_cell.coordinate, Coordinate::new(3, 3));
        }

        #[test]
        fn it_keeps_rest_of_cell() {
            let cell = Cell {
                coordinate: Coordinate::splat(2),
                layers: vec![crate::cells::layer::LayerType::Floor],
            };
            let new_cell = cell.translate(3, 3);

            assert_eq!(new_cell.cell_type(), crate::cells::layer::LayerType::Floor);
            assert!(new_cell.is_spawnable());
            assert!(new_cell.is_walkable());
        }
    }

    mod is_at_location {
        use crate::cells::cell::Cell;

        #[test]
        fn it_returns_true_when_same_coordinates() {
            let cell = Cell::splat(0, crate::cells::layer::LayerType::Empty);
            assert!(cell.is_at_location(0, 0));
        }

        #[test]
        fn it_returns_false_when_same_coordinates() {
            let cell = Cell::splat(0, crate::cells::layer::LayerType::Empty);
            assert!(!cell.is_at_location(1, 0));
        }
    }

    mod new {
        use crate::{cells::cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::new(3, 3, crate::cells::layer::LayerType::Floor);
            assert_eq!(cell.coordinate, Coordinate::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cells::layer::LayerType::Floor);
            assert!(cell.is_spawnable());
            assert!(cell.is_walkable());
        }
    }

    mod splat {
        use crate::{cells::cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::splat(3, crate::cells::layer::LayerType::Floor);
            assert_eq!(cell.coordinate, Coordinate::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cells::layer::LayerType::Floor);
            assert!(cell.is_spawnable());
            assert!(cell.is_walkable());
        }
    }

    mod splatted_room_floor {
        use crate::{cells::cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::splatted_room_floor(3);
            assert_eq!(cell.coordinate, Coordinate::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cells::layer::LayerType::RoomFloor);
            assert!(cell.is_spawnable());
            assert!(cell.is_walkable());
        }
    }

    mod room_floor {
        use crate::{cells::cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::room_floor(3, 3);
            assert_eq!(cell.coordinate, Coordinate::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cells::layer::LayerType::RoomFloor);
            assert!(cell.is_spawnable());
            assert!(cell.is_walkable());
        }
    }

    mod splatted_room_wall {
        use crate::{cells::cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::splatted_room_wall(3);
            assert_eq!(cell.coordinate, Coordinate::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cells::layer::LayerType::RoomWall);
            assert!(!cell.is_spawnable());
            assert!(!cell.is_walkable());
        }
    }

    mod room_wall {
        use crate::{cells::cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::room_wall(3, 3);
            assert_eq!(cell.coordinate, Coordinate::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cells::layer::LayerType::RoomWall);
            assert!(!cell.is_spawnable());
            assert!(!cell.is_walkable());
        }
    }

    mod room_door {
        use crate::{cells::cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::room_door(3, 3);
            assert_eq!(cell.coordinate, Coordinate::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cells::layer::LayerType::Door);
            assert!(!cell.is_spawnable());
            assert!(cell.is_walkable());
        }
    }

    mod outer_wall {
        use crate::{cells::cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::outer_wall(3, 3);
            assert_eq!(cell.coordinate, Coordinate::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cells::layer::LayerType::OuterWall);
            assert!(!cell.is_spawnable());
            assert!(!cell.is_walkable());
        }
    }

    mod empty_cell {
        use crate::{cells::cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::empty_cell(3, 3);
            assert_eq!(cell.coordinate, Coordinate::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cells::layer::LayerType::Empty);
            assert!(!cell.is_spawnable());
            assert!(!cell.is_walkable());
        }
    }
}
