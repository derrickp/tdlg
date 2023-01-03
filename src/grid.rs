use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use std::collections::HashMap;

use crate::{
    cells::{Cell, LayerType},
    coordinate::Coordinate,
    room::Room,
};

pub struct Grid {
    pub cells: HashMap<Coordinate, Cell>,
    pub size: usize,
    pub seed: String,
    pub rng: Pcg64,
}

impl Grid {
    fn add_cell(&mut self, cell: Cell) {
        self.cells.insert(cell.coordinate, cell);
    }

    pub fn add_room(&mut self, room: Room) {
        for cell in room.cells.iter() {
            if let Some(grid_cell) = self.cells.get_mut(&cell.coordinate) {
                grid_cell.clear_contents();
                grid_cell.add_cell_layers(&cell.layers);
            }
        }
    }

    pub fn fill_empty_cells(&mut self) {
        for cell in self.cells.values_mut().filter(|c| c.is_empty()) {
            cell.set_to_floor();
        }
    }

    pub fn is_cell_empty(&self, coordinate: &Coordinate) -> bool {
        let cell = self.cells.get(coordinate);
        match cell {
            Some(c) => c.is_empty(),
            None => false,
        }
    }

    pub fn random_coordinate(&mut self) -> Option<Coordinate> {
        let mut coordinates: Vec<Coordinate> = self.cells.keys().copied().collect();

        coordinates.sort_by_key(|coordinate| (coordinate.x, coordinate.y));
        let index: usize = self.rng.gen_range(0..coordinates.len());
        coordinates.get(index).copied()
    }

    pub fn random_spawnable_coordinate(&mut self) -> Option<Coordinate> {
        let mut spawnable_cells: Vec<Coordinate> = self
            .cells
            .iter()
            .filter_map(|(coordinate, cell)| {
                if cell.is_spawnable() {
                    Some(*coordinate)
                } else {
                    None
                }
            })
            .collect();
        spawnable_cells.sort_by_key(|coordinate| (coordinate.x, coordinate.y));
        let index: usize = self.rng.gen_range(0..spawnable_cells.len());
        spawnable_cells.get(index).copied()
    }

    pub fn add_layer(&mut self, coordinate: &Coordinate, layer: LayerType) {
        if let Some(cell) = self.cells.get_mut(coordinate) {
            cell.add_layer(&layer)
        }
    }

    pub fn bury_layer(&mut self, coordinate: &Coordinate, layer: LayerType) {
        if let Some(cell) = self.cells.get_mut(coordinate) {
            cell.bury_layer(&layer)
        }
    }

    pub fn random_unblocked_coordinate(&mut self) -> Option<Coordinate> {
        let mut coordinates: Vec<Coordinate> = self
            .cells
            .iter_mut()
            .map(|(coordinate, _)| *coordinate)
            .collect();
        coordinates.sort_by_key(|coordinate| (coordinate.x, coordinate.y));
        let index: usize = self.rng.gen_range(0..coordinates.len());
        coordinates.get(index).copied()
    }

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

    pub fn build(size: usize, seed: String) -> Self {
        let mut grid = Self {
            size,
            seed: seed.clone(),
            cells: HashMap::default(),
            rng: Seeder::from(seed.as_str()).make_rng(),
        };

        for x in 0..size as i32 {
            for y in 0..size as i32 {
                grid.add_cell(Cell::empty_cell(x, y));
            }
        }

        grid
    }
}
