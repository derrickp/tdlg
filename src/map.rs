use crate::{coordinate::Coordinate, grid::Grid};

pub struct TopDownMap {
    pub grid: Grid<i32>,
    pub room_count: usize,
    pub entry_coordinate: Coordinate<i32>,
}
