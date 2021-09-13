use crate::coordinate::Coordinate;
use std::hash::Hash;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CellType {
    Door,
    Empty,
    Floor,
    OuterWall,
    RoomFloor,
    RoomWall,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Cell<T: Copy + std::ops::Add<Output = T> + Eq + Hash> {
    pub cell_type: CellType,
    pub coordinate: Coordinate<T>,
    pub spawnable: bool,
    pub walkable: bool,
}

impl<T: Copy + std::ops::Add<Output = T> + Eq + Hash> Cell<T> {
    pub fn is_at_location(&self, x: T, y: T) -> bool {
        return self.coordinate.x == x && self.coordinate.y == y;
    }

    pub fn set_cell_type(&mut self, cell_type: CellType) {
        self.cell_type = cell_type;
        match self.cell_type {
            CellType::Floor => {
                self.spawnable = true;
                self.walkable = true;
            }
            CellType::RoomFloor => {
                self.spawnable = true;
                self.walkable = true;
            }
            CellType::Door => {
                self.spawnable = false;
                self.walkable = true;
            }
            _ => {}
        }
    }

    pub fn translate(&self, x: T, y: T) -> Self {
        return Self {
            coordinate: Coordinate::new(x, y),
            cell_type: self.cell_type,
            spawnable: self.spawnable,
            walkable: self.walkable,
        };
    }

    pub fn new(x: T, y: T, cell_type: CellType, spawnable: bool, walkable: bool) -> Self {
        return Self {
            cell_type,
            spawnable,
            walkable,
            coordinate: Coordinate::new(x, y),
        };
    }

    pub fn splat(value: T, cell_type: CellType, spawnable: bool, walkable: bool) -> Self {
        return Self {
            cell_type,
            spawnable,
            walkable,
            coordinate: Coordinate::splat(value),
        };
    }

    pub fn splatted_room_wall(value: T) -> Self {
        return Self::splat(value, CellType::RoomWall, false, false);
    }

    pub fn room_wall(x: T, y: T) -> Self {
        return Self::new(x, y, CellType::RoomWall, false, false);
    }

    pub fn splatted_room_floor(value: T) -> Self {
        return Self::splat(value, CellType::RoomFloor, true, true);
    }

    pub fn room_floor(x: T, y: T) -> Self {
        return Self::new(x, y, CellType::RoomFloor, true, true);
    }

    pub fn room_door(x: T, y: T) -> Self {
        return Self::new(x, y, CellType::Door, false, true);
    }

    pub fn outer_wall(x: T, y: T) -> Self {
        return Self::new(x, y, CellType::OuterWall, false, false);
    }

    pub fn empty_cell(x: T, y: T) -> Self {
        return Self::new(x, y, CellType::Empty, false, false);
    }
}

#[cfg(test)]
mod tests {
    mod set_cell_type {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_changes_the_cell_type() {
            let mut cell = Cell::<i32> {
                cell_type: crate::cell::CellType::Empty,
                coordinate: Coordinate::splat(2),
                spawnable: false,
                walkable: false,
            };
            cell.set_cell_type(crate::cell::CellType::RoomFloor);
            assert_eq!(cell.cell_type, crate::cell::CellType::RoomFloor);
        }

        #[test]
        fn it_updates_spawnable_walkable_for_room_floor() {
            let mut cell = Cell::<i32> {
                cell_type: crate::cell::CellType::Empty,
                coordinate: Coordinate::splat(2),
                spawnable: false,
                walkable: false,
            };
            cell.set_cell_type(crate::cell::CellType::RoomFloor);
            assert_eq!(cell.spawnable, true);
            assert_eq!(cell.walkable, true);
        }

        #[test]
        fn it_updates_spawnable_walkable_for_floor() {
            let mut cell = Cell::<i32> {
                cell_type: crate::cell::CellType::Empty,
                coordinate: Coordinate::splat(2),
                spawnable: false,
                walkable: false,
            };
            cell.set_cell_type(crate::cell::CellType::Floor);
            assert_eq!(cell.spawnable, true);
            assert_eq!(cell.walkable, true);
        }

        #[test]
        fn it_updates_spawnable_walkable_for_door() {
            let mut cell = Cell::<i32> {
                cell_type: crate::cell::CellType::Empty,
                coordinate: Coordinate::splat(2),
                spawnable: true,
                walkable: false,
            };
            cell.set_cell_type(crate::cell::CellType::Door);
            assert_eq!(cell.spawnable, false);
            assert_eq!(cell.walkable, true);
        }
    }

    mod translate {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_updates_cell_coordinates() {
            let cell = Cell::<i32> {
                cell_type: crate::cell::CellType::Empty,
                coordinate: Coordinate::splat(2),
                spawnable: true,
                walkable: false,
            };
            let new_cell = cell.translate(3, 3);

            assert_eq!(new_cell.coordinate, Coordinate::<i32>::new(3, 3));
        }

        #[test]
        fn it_keeps_rest_of_cell() {
            let cell = Cell::<i32> {
                cell_type: crate::cell::CellType::Floor,
                coordinate: Coordinate::splat(2),
                spawnable: true,
                walkable: true,
            };
            let new_cell = cell.translate(3, 3);

            assert_eq!(new_cell.cell_type, crate::cell::CellType::Floor);
            assert_eq!(new_cell.spawnable, true);
            assert_eq!(new_cell.walkable, true);
        }
    }

    mod is_at_location {
        use crate::cell::Cell;

        #[test]
        fn it_returns_true_when_same_coordinates() {
            let cell = Cell::splat(0, crate::cell::CellType::Empty, false, false);
            assert_eq!(cell.is_at_location(0, 0), true);
        }

        #[test]
        fn it_returns_false_when_same_coordinates() {
            let cell = Cell::splat(0, crate::cell::CellType::Empty, false, false);
            assert_eq!(cell.is_at_location(1, 0), false);
        }
    }

    mod new {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::new(3, 3, crate::cell::CellType::Floor, false, false);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type, crate::cell::CellType::Floor);
            assert_eq!(cell.spawnable, false);
            assert_eq!(cell.walkable, false);
        }
    }

    mod splat {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::splat(3, crate::cell::CellType::Floor, false, false);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type, crate::cell::CellType::Floor);
            assert_eq!(cell.spawnable, false);
            assert_eq!(cell.walkable, false);
        }
    }

    mod splatted_room_floor {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::splatted_room_floor(3);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type, crate::cell::CellType::RoomFloor);
            assert_eq!(cell.spawnable, true);
            assert_eq!(cell.walkable, true);
        }
    }

    mod room_floor {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::room_floor(3, 3);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type, crate::cell::CellType::RoomFloor);
            assert_eq!(cell.spawnable, true);
            assert_eq!(cell.walkable, true);
        }
    }

    mod splatted_room_wall {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::splatted_room_wall(3);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type, crate::cell::CellType::RoomWall);
            assert_eq!(cell.spawnable, false);
            assert_eq!(cell.walkable, false);
        }
    }

    mod room_wall {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::room_wall(3, 3);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type, crate::cell::CellType::RoomWall);
            assert_eq!(cell.spawnable, false);
            assert_eq!(cell.walkable, false);
        }
    }

    mod room_door {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::room_door(3, 3);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type, crate::cell::CellType::Door);
            assert_eq!(cell.spawnable, false);
            assert_eq!(cell.walkable, true);
        }
    }

    mod outer_wall {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::outer_wall(3, 3);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type, crate::cell::CellType::OuterWall);
            assert_eq!(cell.spawnable, false);
            assert_eq!(cell.walkable, false);
        }
    }

    mod empty_cell {
        use crate::{cell::Cell, coordinate::Coordinate};

        #[test]
        fn it_creates_cell_properly() {
            let cell = Cell::<i32>::empty_cell(3, 3);
            assert_eq!(cell.coordinate, Coordinate::<i32>::new(3, 3));
            assert_eq!(cell.cell_type, crate::cell::CellType::Empty);
            assert_eq!(cell.spawnable, false);
            assert_eq!(cell.walkable, false);
        }
    }
}
