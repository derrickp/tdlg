use crate::{grid::Grid, loading::load_room_templates, map::TopDownMap};
use rand::Rng;

pub struct Generator {
    pub grid_size: usize,
    pub target_number_rooms: usize,
    pub room_template_directory: &'static str,
}

impl Generator {
    pub fn generate_top_down_map(&self) -> TopDownMap {
        let mut rng = rand::thread_rng();

        let mut grid = Grid::build(self.grid_size);
        let mut room_count = 0;

        if let Some(room_templates) = load_room_templates(self.room_template_directory) {
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
        }

        grid.fill_empty_cells();
        grid.create_outer_wall();

        let entry_coordinate = grid.random_spawnable_coordinate().unwrap();

        TopDownMap {
            grid,
            room_count,
            entry_coordinate,
        }
    }
}
