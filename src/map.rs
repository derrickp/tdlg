use crate::{coordinate::Coordinate, grid::Grid};

pub struct TopDownMap {
    pub grid: Grid,
    pub room_count: usize,
    pub entry_coordinate: Coordinate,
}
