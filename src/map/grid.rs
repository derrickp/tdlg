use super::{
    cells::{Cell, Coordinate},
    layers::{LayerType, StructureType},
    Room,
};
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Grid {
    cells: HashMap<Coordinate, Cell>,
    size: u16,
    rng: Pcg64,
}

impl Grid {
    pub fn cell(&self, coordinate: &Coordinate) -> Option<&Cell> {
        self.cells.get(coordinate)
    }

    pub fn cells(&self) -> Vec<&Cell> {
        self.cells.values().collect()
    }

    pub fn top_layer_display(&self) -> String {
        let mut text = String::new();

        for y in (-1..=(self.size as i32)).rev() {
            for x in -1..=(self.size as i32) {
                let coordinate = Coordinate::from((x, y));
                let layer_type = self
                    .cells
                    .get(&coordinate)
                    .map(|c| c.visible_layer())
                    .unwrap_or_default();
                text.push(layer_type.into());
            }

            text.push('\n');
        }

        text
    }

    fn add_cell(&mut self, cell: Cell) {
        self.cells.insert(*cell.coordinate(), cell);
    }

    pub fn surrounding_walkable_coordinates(&self, coordinate: &Coordinate) -> Vec<Coordinate> {
        let surrounding: Vec<Coordinate> = vec![
            // Coordinate::new(coordinate.x - 1, coordinate.y - 1),
            Coordinate::from((coordinate.x() - 1, coordinate.y())),
            // Coordinate::new(coordinate.x - 1, coordinate.y + 1),
            Coordinate::from((coordinate.x(), coordinate.y() + 1)),
            // Coordinate::new(coordinate.x + 1, coordinate.y + 1),
            Coordinate::from((coordinate.x() + 1, coordinate.y())),
            // Coordinate::new(coordinate.x + 1, coordinate.y - 1),
            Coordinate::from((coordinate.x(), coordinate.y() - 1)),
        ];
        surrounding
            .iter()
            .filter_map(|c| self.cells.get(c).filter(|cell| cell.is_walkable()))
            .map(|cell| cell.coordinate())
            .copied()
            .collect()
    }

    pub fn add_room(&mut self, room: Room) {
        for cell in room.cells().iter() {
            if let Some(grid_cell) = self.cells.get_mut(cell.coordinate()) {
                grid_cell.clear_contents();
                for layer in cell.layers().iter() {
                    grid_cell.add_layer(*layer);
                }
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

        coordinates.sort_by_key(|coordinate| (coordinate.x(), coordinate.y()));
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
        spawnable_cells.sort_by_key(|coordinate| (coordinate.x(), coordinate.y()));
        let index: usize = self.rng.gen_range(0..spawnable_cells.len());
        spawnable_cells.get(index).copied()
    }

    pub fn add_layer(&mut self, coordinate: &Coordinate, layer: LayerType) {
        if let Some(cell) = self.cells.get_mut(coordinate) {
            cell.add_layer(layer)
        }
    }

    pub fn remove_layer(&mut self, coordinate: &Coordinate, layer: LayerType) {
        if let Some(cell) = self.cells.get_mut(coordinate) {
            cell.remove_layer(&layer)
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
        coordinates.sort_by_key(|coordinate| (coordinate.x(), coordinate.y()));
        let index: usize = self.rng.gen_range(0..coordinates.len());
        coordinates.get(index).copied()
    }

    pub fn create_outer_wall(&mut self) {
        // X rows
        for x in -1..=self.size as i32 {
            self.add_cell(Cell::from((
                Coordinate::from((x, -1)),
                LayerType::Structure(StructureType::Boulder),
            )));
            self.add_cell(Cell::from((
                Coordinate::from((x, self.size as i32)),
                LayerType::Structure(StructureType::Boulder),
            )));
        }

        // Y rows
        for y in 0..=self.size as i32 {
            self.add_cell(Cell::from((
                Coordinate::from((-1, y)),
                LayerType::Structure(StructureType::Boulder),
            )));
            self.add_cell(Cell::from((
                Coordinate::from((self.size as i32, y)),
                LayerType::Structure(StructureType::Boulder),
            )));
        }
    }

    pub fn build(size: u16, seed: String) -> Self {
        let rng: Pcg64 = Seeder::from(seed.as_str()).make_rng();
        let mut grid = Self {
            size,
            cells: HashMap::default(),
            rng,
        };

        for x in 0..size as i32 {
            for y in 0..size as i32 {
                grid.add_cell(Cell::from((x, y)));
            }
        }

        grid
    }
}
