use super::{cells::Coordinate, Grid};

#[derive(Debug)]
pub struct TopDownMap {
    grid: Grid,
    room_count: usize,
    entry_coordinate: Coordinate,
    exit_coordinate: Coordinate,
}

impl TopDownMap {
    pub fn new(
        grid: Grid,
        room_count: usize,
        entry_coordinate: Coordinate,
        exit_coordinate: Coordinate,
    ) -> Self {
        Self {
            grid,
            room_count,
            entry_coordinate,
            exit_coordinate,
        }
    }

    pub fn room_count(&self) -> usize {
        self.room_count
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    pub fn grid_mut(&mut self) -> &mut Grid {
        &mut self.grid
    }

    pub fn entry(&self) -> &Coordinate {
        &self.entry_coordinate
    }

    pub fn exit(&self) -> &Coordinate {
        &self.exit_coordinate
    }
}
