use rand::Rng;
use std::{collections::HashMap, hash::Hash};

use crate::{
    cell::{Cell, CellType},
    coordinate::Coordinate,
    room::Room,
};

pub struct Grid<T: Copy + std::ops::Add<Output = T> + Eq + Hash> {
    pub cells: HashMap<Coordinate<T>, Cell<T>>,
    pub size: usize,
}

impl<T: Copy + std::ops::Add<Output = T> + Eq + Hash> Grid<T> {
    fn add_cell(&mut self, cell: Cell<T>) {
        self.cells.insert(cell.coordinate, cell);
    }

    fn set_cell_type(&mut self, x: T, y: T, cell_type: CellType) {
        let coordinate = Coordinate { x, y };
        if let Some(cell) = self.cells.get_mut(&coordinate) {
            cell.set_cell_type(cell_type);
        };
    }

    pub fn add_room(&mut self, room: Room<T>) {
        for cell in room.cells.iter() {
            self.set_cell_type(cell.coordinate.x, cell.coordinate.y, cell.cell_type);
        }
    }

    pub fn fill_empty_cells(&mut self) {
        for cell in self.cells.values_mut().filter(|c| c.is_empty()) {
            cell.set_to_floor();
        }
    }

    pub fn is_cell_empty(&self, coordinate: &Coordinate<T>) -> bool {
        let cell = self.cells.get(coordinate);
        match cell {
            Some(c) => c.is_empty(),
            None => false,
        }
    }

    pub fn random_spawnable_coordinate(&self) -> Option<Coordinate<T>> {
        let mut rng = rand::thread_rng();
        let spawnable_cells: Vec<Coordinate<T>> = self
            .cells
            .iter()
            .filter_map(|(coordinate, cell)| {
                if cell.spawnable {
                    Some(coordinate.clone())
                } else {
                    None
                }
            })
            .collect();
        let index: usize = rng.gen_range(0..spawnable_cells.len());
        if let Some(coordinate) = spawnable_cells.get(index) {
            return Some(coordinate.clone());
        } else {
            return None;
        }
    }
}

impl Grid<i32> {
    pub fn create_outer_wall(&mut self) {
        // X rows
        for x in -1..=self.size as i32 {
            self.add_cell(Cell::outer_wall(x, -1));
            self.add_cell(Cell::outer_wall(x, self.size as i32));
        }

        // Y rows
        for y in 0..=self.size as i32 {
            self.add_cell(Cell::outer_wall(-1, y));
            self.add_cell(Cell::outer_wall(self.size as i32, y));
        }
    }

    pub fn build(size: usize) -> Self {
        let mut grid = Self {
            size,
            cells: HashMap::default(),
        };

        for x in 0..size as i32 {
            for y in 0..size as i32 {
                grid.add_cell(Cell::empty_cell(x, y));
            }
        }

        return grid;
    }
}
