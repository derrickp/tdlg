use crate::{grid::Grid, loading::RoomPaths, map::TopDownMap, room::Room};
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

pub struct Generator {
    pub grid_size: usize,
    pub target_number_rooms: usize,
    pub all_room_paths: Vec<RoomPaths>,
    pub seed: String,
}

#[derive(Debug)]
pub struct GenerationError {
    pub message: String,
}

impl GenerationError {
    fn no_room_paths() -> Self {
        Self {
            message: "no_room_paths".to_string(),
        }
    }

    fn room_templates_cannot_be_loaded() -> Self {
        Self {
            message: "room_templates_cannot_be_loaded".to_string(),
        }
    }
}

impl Generator {
    pub fn generate_top_down_map(&self) -> Result<TopDownMap, GenerationError> {
        if self.all_room_paths.is_empty() {
            return Err(GenerationError::no_room_paths());
        }

        let room_iterator = self
            .all_room_paths
            .iter()
            .map(|f| f.load_rooms().into_iter().flatten())
            .flatten();
        let mut room_templates: Vec<Room> = Vec::new();

        for room in room_iterator {
            room_templates.push(room);
        }

        if room_templates.is_empty() {
            return Err(GenerationError::room_templates_cannot_be_loaded());
        }

        let mut rng: Pcg64 = Seeder::from(self.seed.as_str()).make_rng();

        let mut grid = Grid::build(self.grid_size, self.seed.clone());
        let mut room_count = 0;

        for _ in 0..self.target_number_rooms {
            let index: usize = rng.gen_range(0..room_templates.len());
            let template = room_templates.get(index).unwrap().clone();
            let max_side_length = template.max_side_length;

            let x: i32 = rng.gen_range(1..=(self.grid_size - max_side_length) as i32);
            let y: i32 = rng.gen_range(1..=(self.grid_size - max_side_length - 1) as i32);
            let room = template.translate(x, y);

            if room
                .cells
                .iter()
                .all(|cell| grid.is_cell_empty(&cell.coordinate))
            {
                room_count += 1;
                grid.add_room(room);
            }
        }

        grid.fill_empty_cells();
        grid.create_outer_wall();

        let entry_coordinate = grid.random_spawnable_coordinate().unwrap();

        Ok(TopDownMap {
            grid,
            room_count,
            entry_coordinate,
        })
    }
}
