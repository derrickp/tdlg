use std::num::NonZeroU16;

use super::{assets::RoomTemplates, Generator, ItemGeneration};

#[derive(Default, Clone)]
pub struct GeneratorBuilder {
    grid_size: Option<NonZeroU16>,
    target_number_rooms: Option<NonZeroU16>,
    room_templates: Option<RoomTemplates>,
    seed: Option<String>,
    target_hidden_items: Option<ItemGeneration>,
    target_items: Option<ItemGeneration>,
}

pub fn builder() -> GeneratorBuilder {
    GeneratorBuilder::default()
}

const DEFAULT_SEED: &str = "tdlg";
const DEFAULT_GRID_SIZE: u16 = 100;
const DEFAULT_TARGET_NUMBER_ROOMS: u16 = 25;

impl GeneratorBuilder {
    pub fn build(&self) -> Generator {
        let seed = self
            .seed
            .to_owned()
            .unwrap_or_else(|| DEFAULT_SEED.to_string());
        let grid_size = self
            .grid_size
            .unwrap_or_else(|| NonZeroU16::new(DEFAULT_GRID_SIZE).unwrap());

        let target_number_rooms = self
            .target_number_rooms
            .unwrap_or_else(|| NonZeroU16::new(DEFAULT_TARGET_NUMBER_ROOMS).unwrap());

        Generator {
            grid_size,
            target_number_rooms,
            seed,
            room_templates: self.room_templates.to_owned().unwrap_or_default(),
            target_hidden_items: self.target_hidden_items.clone(),
            target_items: self.target_items.clone(),
        }
    }

    pub fn seed(&mut self, seed: &str) -> &mut GeneratorBuilder {
        self.seed = Some(seed.to_string());

        self
    }

    pub fn grid_size(&mut self, grid_size: NonZeroU16) -> &mut GeneratorBuilder {
        self.grid_size = Some(grid_size);

        self
    }

    pub fn target_number_rooms(
        &mut self,
        target_number_rooms: NonZeroU16,
    ) -> &mut GeneratorBuilder {
        self.target_number_rooms = Some(target_number_rooms);

        self
    }

    pub fn room_templates(&mut self, room_templates: RoomTemplates) -> &mut GeneratorBuilder {
        self.room_templates = Some(room_templates);

        self
    }

    pub fn target_hidden_items(
        &mut self,
        target_hidden_items: ItemGeneration,
    ) -> &mut GeneratorBuilder {
        self.target_hidden_items = Some(target_hidden_items);

        self
    }

    pub fn target_items(&mut self, target_items: ItemGeneration) -> &mut GeneratorBuilder {
        self.target_items = Some(target_items);

        self
    }
}
