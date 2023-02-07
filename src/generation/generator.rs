use std::num::NonZeroU16;

use pathfinding::prelude::astar;
use rand::Rng;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

use crate::{
    loading::RoomPaths,
    map::{layers::LayerType, Grid, Room, TopDownMap},
};

use super::{assets::RoomTemplates, GenerationError, ItemGeneration};

#[derive(Debug)]
pub struct Generator {
    pub grid_size: NonZeroU16,
    pub target_number_rooms: NonZeroU16,
    pub room_templates: RoomTemplates,
    pub seed: String,
    pub target_hidden_items: Option<ItemGeneration>,
    pub target_items: Option<ItemGeneration>,
    rng: Pcg64,
}

const DEFAULT_SEED: &str = "tdlg";
const DEFAULT_GRID_SIZE: u16 = 100;
const DEFAULT_TARGET_NUMBER_ROOMS: u16 = 25;

impl Default for Generator {
    fn default() -> Self {
        Self {
            grid_size: NonZeroU16::new(DEFAULT_GRID_SIZE).unwrap(),
            target_number_rooms: NonZeroU16::new(DEFAULT_TARGET_NUMBER_ROOMS).unwrap(),
            room_templates: RoomTemplates::default(),
            seed: DEFAULT_SEED.to_string(),
            target_hidden_items: Default::default(),
            target_items: Default::default(),
            rng: Seeder::from(DEFAULT_SEED).make_rng(),
        }
    }
}

const CHANCE_TO_SPAWN_IN_ROOM: u8 = 25;

impl Generator {
    pub fn new(
        seed: &str,
        grid_size: NonZeroU16,
        target_number_rooms: NonZeroU16,
        target_items: Option<ItemGeneration>,
        target_hidden_items: Option<ItemGeneration>,
        room_templates: RoomTemplates,
    ) -> Self {
        Generator {
            grid_size,
            target_number_rooms,
            room_templates,
            seed: seed.to_string(),
            target_hidden_items,
            target_items,
            rng: Seeder::from(seed).make_rng(),
        }
    }

    pub fn build(seed: &str, grid_size: NonZeroU16, target_number_rooms: NonZeroU16) -> Self {
        Generator {
            grid_size,
            target_number_rooms,
            room_templates: RoomTemplates::default(),
            seed: seed.to_string(),
            target_hidden_items: Default::default(),
            target_items: Default::default(),
            rng: Seeder::from(seed).make_rng(),
        }
    }

    pub fn load(
        seed: &str,
        all_room_paths: Vec<RoomPaths>,
        target_hidden_items: Option<ItemGeneration>,
        target_items: Option<ItemGeneration>,
    ) -> Result<Generator, GenerationError> {
        if all_room_paths.is_empty() {
            return Err(GenerationError::no_room_paths());
        }

        let rooms: Vec<Room> = all_room_paths
            .into_iter()
            .flat_map(|path| path.load_rooms().into_iter().flatten())
            .collect();

        Ok(Generator {
            grid_size: NonZeroU16::new(DEFAULT_GRID_SIZE).unwrap(),
            target_number_rooms: NonZeroU16::new(DEFAULT_TARGET_NUMBER_ROOMS).unwrap(),
            room_templates: RoomTemplates { rooms },
            seed: seed.to_string(),
            target_hidden_items,
            target_items,
            rng: Seeder::from(seed).make_rng(),
        })
    }

    pub fn generate_top_down_map(&mut self) -> Result<TopDownMap, GenerationError> {
        if self.room_templates.rooms.is_empty() {
            return Err(GenerationError::no_room_paths());
        }

        let mut grid = Grid::build(self.grid_size.get(), self.seed.clone());
        let mut room_count = 0;

        for _ in 0..self.target_number_rooms.get() {
            let index: usize = self.rng.gen_range(0..self.room_templates.rooms.len());
            let template = self.room_templates.rooms.get(index).unwrap().clone();
            let max_side_length = template.max_side_length();

            let x: i32 = self
                .rng
                .gen_range(1..=(self.grid_size.get() - max_side_length) as i32);
            let y: i32 = self
                .rng
                .gen_range(1..=(self.grid_size.get() - max_side_length - 1) as i32);
            let mut room = template.translate(x, y);

            let door_cells = room.possible_door_cells();
            if door_cells.is_empty() {
                continue;
            }

            let index = self.rng.gen_range(0..door_cells.len());
            if let Some(cell) = door_cells.get(index) {
                room.replace_cell_contents(
                    cell.coordinate().x(),
                    cell.coordinate().y(),
                    LayerType::Door,
                );
            } else {
                println!("did not find door {index} {door_cells:?}");
            }

            let roll_for_spawn: u8 = self.rng.gen_range(1..=100);
            if roll_for_spawn <= CHANCE_TO_SPAWN_IN_ROOM {
                let spawnable_cells = room.spawnable_cells();
                let spawn_index_range = 0..spawnable_cells.len();
                if !spawn_index_range.is_empty() {
                    if let Some(cell) = spawnable_cells.get(self.rng.gen_range(spawn_index_range)) {
                        room.add_layer_to_cell(
                            cell.coordinate().x(),
                            cell.coordinate().y(),
                            LayerType::Table,
                        );
                    }
                }
            }

            if room
                .cells()
                .iter()
                .all(|cell| grid.is_cell_empty(cell.coordinate()))
            {
                room_count += 1;
                grid.add_room(room);
            }
        }

        grid.fill_empty_cells();
        grid.create_outer_wall();

        let entry_coordinate = grid.random_spawnable_coordinate().unwrap();

        if let Some(hidden_item_generation) = &self.target_hidden_items {
            for _ in 0..hidden_item_generation.target_num_items {
                let coordinate = grid.random_unblocked_coordinate().unwrap();
                let chance: usize = self.rng.gen_range(0..100);
                if let Some(it) = hidden_item_generation
                    .item_ranges
                    .iter()
                    .find(|hidden_chance| hidden_chance.chance.contains(&chance))
                {
                    grid.bury_layer(&coordinate, it.layer_type)
                }
            }
        }

        if let Some(item_generation) = &self.target_items {
            for _ in 0..item_generation.target_num_items {
                let coordinate = grid.random_spawnable_coordinate().unwrap();
                let chance: usize = self.rng.gen_range(0..100);
                if let Some(it) = item_generation
                    .item_ranges
                    .iter()
                    .find(|item_chance| item_chance.chance.contains(&chance))
                {
                    grid.add_layer(&coordinate, it.layer_type)
                }
            }
        }

        let exit_coordinate = grid.random_spawnable_coordinate().unwrap();

        grid.add_layer(&entry_coordinate, LayerType::Entrance);
        grid.add_layer(&exit_coordinate, LayerType::Exit);

        let result = astar(
            &entry_coordinate,
            |c| {
                grid.surrounding_walkable_coordinates(c)
                    .into_iter()
                    .map(|c| (c, 1))
            },
            |c| c.distance(&exit_coordinate) / 3,
            |c| c.eq(&exit_coordinate),
        );

        if let Some((exit_path, _)) = result {
            for coordinate in exit_path.iter() {
                if coordinate.ne(&entry_coordinate) && coordinate.ne(&exit_coordinate) {
                    grid.add_layer(coordinate, LayerType::Path);
                }
            }
        }

        Ok(TopDownMap::new(
            grid,
            room_count,
            entry_coordinate,
            exit_coordinate,
        ))
    }
}
