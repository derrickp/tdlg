use crate::coordinate::Coordinate;
use std::hash::Hash;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CellLayerType {
    Door,
    Empty,
    Floor,
    OuterWall,
    RoomFloor,
    RoomWall,
    Rubble,
    Table,
}

impl CellLayerType {
    pub fn is_walkable(&self) -> bool {
        self == &CellLayerType::Door
            || self == &CellLayerType::RoomFloor
            || self == &CellLayerType::Floor
            || self == &CellLayerType::Rubble
    }

    pub fn is_spawnable(&self) -> bool {
        self == &CellLayerType::RoomFloor || self == &CellLayerType::Floor
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Cell<T: Copy + std::ops::Add<Output = T> + Eq + Hash> {
    pub coordinate: Coordinate<T>,
    pub layers: Vec<CellLayerType>,
}

impl<T: Copy + std::ops::Add<Output = T> + Eq + Hash> Cell<T> {
    pub fn clear_contents(&mut self) {
        self.layers.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.cell_type() == CellLayerType::Empty
    }

    pub fn set_to_floor(&mut self) {
        self.layers.clear();
        self.layers.push(CellLayerType::Floor);
    }

    pub fn is_at_location(&self, x: T, y: T) -> bool {
        self.coordinate.x == x && self.coordinate.y == y
    }

    pub fn cell_type(&self) -> CellLayerType {
        if self.layers.is_empty() {
            return CellLayerType::Empty;
        }

        self.cell_type_at_layer(0).unwrap()
    }

    pub fn cell_type_at_layer(&self, layer_index: usize) -> Option<CellLayerType> {
        if let Some(cell_type) = self.layers.get(layer_index) {
            return Some(*cell_type);
        }

        None
    }

    pub fn add_layer(&mut self, layer: &CellLayerType) {
        // We don't need empty cells
        if layer != &CellLayerType::Empty {
            self.layers.push(*layer);
        }
    }

    pub fn add_cell_layers(&mut self, layers: &[CellLayerType]) {
        for layer in layers {
            self.layers.push(*layer)
        }
    }

    pub fn set_cell_type(&mut self, cell_type: CellLayerType) {
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

    pub fn translate(&self, x: T, y: T) -> Self {
        Self {
            coordinate: Coordinate::new(x, y),
            layers: self.layers.clone(),
        }
    }

    pub fn new(x: T, y: T, cell_type: CellLayerType) -> Self {
        let layers: Vec<CellLayerType> = if cell_type != CellLayerType::Empty {
            vec![cell_type]
        } else {
            Vec::new()
        };

        Self {
            layers,
            coordinate: Coordinate::new(x, y),
        }
    }

    pub fn splat(value: T, cell_type: CellLayerType) -> Self {
        Self {
            coordinate: Coordinate::splat(value),
            layers: vec![cell_type],
        }
    }

    pub fn splatted_room_wall(value: T) -> Self {
        Self::splat(value, CellLayerType::RoomWall)
    }

    pub fn room_wall(x: T, y: T) -> Self {
        Self::new(x, y, CellLayerType::RoomWall)
    }

    pub fn splatted_room_floor(value: T) -> Self {
        Self::splat(value, CellLayerType::RoomFloor)
    }

    pub fn room_floor(x: T, y: T) -> Self {
        Self::new(x, y, CellLayerType::RoomFloor)
    }

    pub fn room_door(x: T, y: T) -> Self {
        Self::new(x, y, CellLayerType::Door)
    }

    pub fn outer_wall(x: T, y: T) -> Self {
        Self::new(x, y, CellLayerType::OuterWall)
    }

    pub fn empty_cell(x: T, y: T) -> Self {
        Self::new(x, y, CellLayerType::Empty)
    }
}

#[cfg(test)]
mod tests {
    mod set_cell_type {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_changes_the_cell_type() {
            let mut cell = Cell::<i32> {
                coordinate: Coordinate::splat(2),
                layers: vec![],
            };
            cell.set_cell_type(crate::cell::CellLayerType::RoomFloor);
            assert_eq!(cell.cell_type(), crate::cell::CellLayerType::RoomFloor);
        }

        #[test]
        fn it_updates_spawnable_walkable_for_room_floor() {
            let mut cell = Cell::<i32> {
                coordinate: Coordinate::splat(2),
                layers: vec![],
            };
            cell.set_cell_type(crate::cell::CellLayerType::RoomFloor);
            assert!(cell.is_spawnable());
            assert!(cell.is_walkable());
        }

        #[test]
        fn it_updates_spawnable_walkable_for_floor() {
            let mut cell = Cell::<i32> {
                coordinate: Coordinate::splat(2),
                layers: vec![],
            };
            cell.set_cell_type(crate::cell::CellLayerType::Floor);
            assert!(cell.is_spawnable());
            assert!(cell.is_walkable());
        }

        #[test]
        fn it_updates_spawnable_walkable_for_door() {
            let mut cell = Cell::<i32> {
                coordinate: Coordinate::splat(2),
                layers: vec![],
            };
            cell.set_cell_type(crate::cell::CellLayerType::Door);
            assert!(!cell.is_spawnable());
            assert!(cell.is_walkable());
        }
    }

    mod translate {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_updates_cell_coordinates() {
            let cell = Cell::<i32> {
                coordinate: Coordinate::splat(2),
                layers: vec![],
            };
            let new_cell = cell.translate(3, 3);

            assert_eq!(new_cell.coordinate, Coordinate::<i32>::new(3, 3));
        }

        #[test]
        fn it_keeps_rest_of_cell() {
            let cell = Cell::<i32> {
                coordinate: Coordinate::splat(2),
                layers: vec![crate::cell::CellLayerType::Floor],
            };
            let new_cell = cell.translate(3, 3);

            assert_eq!(new_cell.cell_type(), crate::cell::CellLayerType::Floor);
            assert!(new_cell.is_spawnable());
            assert!(new_cell.is_walkable());
        }
    }

    mod is_at_location {
        use crate::cell::Cell;

        #[test]
        fn it_returns_true_when_same_coordinates() {
            let cell = Cell::splat(0, crate::cell::CellLayerType::Empty);
            assert!(cell.is_at_location(0, 0));
        }

        #[test]
        fn it_returns_false_when_same_coordinates() {
            let cell = Cell::splat(0, crate::cell::CellLayerType::Empty);
            assert!(!cell.is_at_location(1, 0));
        }
    }

    mod new {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::new(3, 3, crate::cell::CellLayerType::Floor);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cell::CellLayerType::Floor);
            assert!(cell.is_spawnable());
            assert!(cell.is_walkable());
        }
    }

    mod splat {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::splat(3, crate::cell::CellLayerType::Floor);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cell::CellLayerType::Floor);
            assert!(cell.is_spawnable());
            assert!(cell.is_walkable());
        }
    }

    mod splatted_room_floor {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::splatted_room_floor(3);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cell::CellLayerType::RoomFloor);
            assert!(cell.is_spawnable());
            assert!(cell.is_walkable());
        }
    }

    mod room_floor {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::room_floor(3, 3);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cell::CellLayerType::RoomFloor);
            assert!(cell.is_spawnable());
            assert!(cell.is_walkable());
        }
    }

    mod splatted_room_wall {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::splatted_room_wall(3);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cell::CellLayerType::RoomWall);
            assert!(!cell.is_spawnable());
            assert!(!cell.is_walkable());
        }
    }

    mod room_wall {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::room_wall(3, 3);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cell::CellLayerType::RoomWall);
            assert!(!cell.is_spawnable());
            assert!(!cell.is_walkable());
        }
    }

    mod room_door {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::room_door(3, 3);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cell::CellLayerType::Door);
            assert!(!cell.is_spawnable());
            assert!(cell.is_walkable());
        }
    }

    mod outer_wall {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::outer_wall(3, 3);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cell::CellLayerType::OuterWall);
            assert!(!cell.is_spawnable());
            assert!(!cell.is_walkable());
        }
    }

    mod empty_cell {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::empty_cell(3, 3);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type(), crate::cell::CellLayerType::Empty);
            assert!(!cell.is_spawnable());
            assert!(!cell.is_walkable());
        }
    }
}
