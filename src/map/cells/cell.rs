use serde::{Deserialize, Serialize};

use crate::map::layers::{FloorType, LayerType, StructureType};

use super::Coordinate;

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct Cell {
    coordinate: Coordinate,
    layers: Vec<LayerType>,
}

impl From<Coordinate> for Cell {
    fn from(coordinate: Coordinate) -> Self {
        Self {
            coordinate,
            layers: Vec::new(),
        }
    }
}

impl From<(Coordinate, LayerType)> for Cell {
    fn from((coordinate, layer): (Coordinate, LayerType)) -> Self {
        Self {
            coordinate,
            layers: vec![layer],
        }
    }
}

impl From<(i32, i32)> for Cell {
    fn from(value: (i32, i32)) -> Self {
        Coordinate::from(value).into()
    }
}

impl From<i32> for Cell {
    fn from(value: i32) -> Self {
        Coordinate::from(value).into()
    }
}

impl Cell {
    pub fn layers(&self) -> &Vec<LayerType> {
        &self.layers
    }

    pub fn coordinate(&self) -> &Coordinate {
        &self.coordinate
    }

    pub fn clear_contents(&mut self) {
        self.layers.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.visible_layer() == LayerType::Empty
    }

    pub fn set_to_floor(&mut self) {
        self.layers.clear();
        self.layers.push(LayerType::Floor(FloorType::Outdoor));
    }

    pub fn is_at_location(&self, x: i32, y: i32) -> bool {
        self.coordinate.x() == x && self.coordinate.y() == y
    }

    pub fn visible_layer(&self) -> LayerType {
        self.layers.last().copied().unwrap_or_default()
    }

    pub fn is_layer_underground(&self, layer: &LayerType) -> Option<bool> {
        let layer_position = match self.layers.iter().position(|l| l == layer) {
            Some(it) => it,
            _ => return None,
        };

        if LayerType::can_bury_other_layers().contains(layer) {
            return Some(false);
        }

        let underground = LayerType::can_bury_other_layers().iter().any(|bury_layer| {
            match self.layers.iter().position(|l| l == bury_layer) {
                Some(it) => it > layer_position,
                _ => false,
            }
        });

        Some(underground)
    }

    pub fn cell_type(&self) -> LayerType {
        if self.layers.is_empty() {
            return LayerType::Empty;
        }

        self.cell_type_at_layer(0).unwrap()
    }

    pub fn contains_door(&self) -> bool {
        self.layers
            .iter()
            .any(|layer| matches!(layer, LayerType::Structure(StructureType::Door)))
    }

    pub fn cell_type_at_layer(&self, layer_index: usize) -> Option<LayerType> {
        if let Some(cell_type) = self.layers.get(layer_index) {
            return Some(*cell_type);
        }

        None
    }

    pub fn add_layer(&mut self, layer: LayerType) {
        // We don't need empty cells
        if layer != LayerType::Empty {
            if layer == LayerType::Structure(StructureType::Wall)
                && !self
                    .layers
                    .iter()
                    .any(|existing_layer| existing_layer.eq(&LayerType::Floor(FloorType::Indoor)))
            {
                self.layers.push(LayerType::Floor(FloorType::Indoor));
            }

            self.layers.push(layer);
        }
    }

    pub fn bury_layer(&mut self, layer: &LayerType) {
        if layer != &LayerType::Empty {
            self.layers.insert(0, *layer);
        }
    }

    pub fn is_walkable(&self) -> bool {
        self.visible_layer().is_walkable()
    }

    pub fn is_spawnable(&self) -> bool {
        if self.layers.is_empty() {
            false
        } else {
            self.layers.iter().all(|layer| layer.is_spawnable())
        }
    }

    pub fn is_obstructed(&self) -> bool {
        if self.layers.is_empty() {
            true
        } else {
            self.layers
                .iter()
                .any(|layer| layer.is_completely_obstructed())
        }
    }

    pub fn translate(&self, x: i32, y: i32) -> Self {
        Self {
            coordinate: Coordinate::from((x, y)),
            layers: self.layers.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::map::{
        cells::{Cell, Coordinate},
        layers::{FloorType, LayerType, StructureType},
    };

    #[test]
    fn contains_door_with_door() {
        let cell = Cell {
            coordinate: Coordinate::from(2),
            layers: vec![
                LayerType::Structure(StructureType::Door),
                LayerType::Floor(FloorType::Outdoor),
            ],
        };
        assert!(cell.contains_door());
    }

    #[test]
    fn contains_door_without_door() {
        let cell = Cell {
            coordinate: Coordinate::from(2),
            layers: vec![
                LayerType::Structure(StructureType::Wall),
                LayerType::Floor(FloorType::Outdoor),
            ],
        };
        assert!(!cell.contains_door());
    }

    #[test]
    fn add_layer_type() {
        let mut cell = Cell {
            coordinate: Coordinate::from(2),
            layers: vec![],
        };
        cell.add_layer(LayerType::Floor(FloorType::Indoor));
        assert_eq!(cell.visible_layer(), LayerType::Floor(FloorType::Indoor));
    }

    #[test]
    fn cell_reports_spawnable_walkable_after_add() {
        let mut cell = Cell {
            coordinate: Coordinate::from(2),
            layers: vec![],
        };
        cell.add_layer(LayerType::Floor(FloorType::Indoor));
        assert!(cell.is_spawnable());
        assert!(cell.is_walkable());
    }

    #[test]
    fn it_updates_spawnable_walkable_for_door() {
        let mut cell = Cell {
            coordinate: Coordinate::from(2),
            layers: vec![],
        };
        cell.add_layer(LayerType::Structure(StructureType::Door));
        assert!(!cell.is_spawnable());
        assert!(cell.is_walkable());
    }

    #[test]
    fn it_updates_cell_coordinates() {
        let cell = Cell {
            coordinate: Coordinate::from(2),
            layers: vec![],
        };
        let new_cell = cell.translate(3, 3);

        assert_eq!(new_cell.coordinate, Coordinate::from((3, 3)));
    }

    #[test]
    fn it_keeps_rest_of_cell() {
        let cell = Cell {
            coordinate: Coordinate::from(2),
            layers: vec![LayerType::Floor(FloorType::Outdoor)],
        };
        let new_cell = cell.translate(3, 3);

        assert_eq!(new_cell.cell_type(), LayerType::Floor(FloorType::Outdoor));
        assert!(new_cell.is_spawnable());
        assert!(new_cell.is_walkable());
    }

    #[test]
    fn is_at_location_with_same_coords_is_true() {
        let cell = Cell::from((Coordinate::from(0), LayerType::Empty));
        assert!(cell.is_at_location(0, 0));
    }

    #[test]
    fn is_at_location_with_other_coords_is_false() {
        let cell = Cell::from((Coordinate::from(0), LayerType::Empty));
        assert!(!cell.is_at_location(1, 0));
    }

    #[test]
    fn new_creates_cell_properly() {
        let cell = Cell::from((Coordinate::from(3), LayerType::Floor(FloorType::Outdoor)));
        assert_eq!(cell.coordinate, Coordinate::from(3));
        assert_eq!(cell.cell_type(), LayerType::Floor(FloorType::Outdoor));
        assert!(cell.is_spawnable());
        assert!(cell.is_walkable());
    }

    #[test]
    fn from_creates_cell_properly_with_layer() {
        let cell = Cell::from((Coordinate::from(3), LayerType::Floor(FloorType::Outdoor)));
        assert_eq!(cell.coordinate, Coordinate::from((3, 3)));
        assert_eq!(cell.cell_type(), LayerType::Floor(FloorType::Outdoor));
        assert!(cell.is_spawnable());
        assert!(cell.is_walkable());
    }
}
