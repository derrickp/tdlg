use serde::{Deserialize, Serialize};

use super::{item_rarity::ItemRarity, FloorType, StructureType};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LayerType {
    Empty,
    Entrance,
    Exit,
    Item(ItemRarity),
    Floor(FloorType),
    Note,
    Path,
    Structure(StructureType),
}

impl Default for LayerType {
    fn default() -> Self {
        Self::Empty
    }
}

impl From<LayerType> for char {
    fn from(val: LayerType) -> Self {
        match val {
            LayerType::Item(ItemRarity::Common) => 'c',
            LayerType::Structure(StructureType::Door) => 'D',
            LayerType::Empty => '_',
            LayerType::Entrance => 'o',
            LayerType::Exit => 'x',
            LayerType::Item(ItemRarity::Exotic) => 'i',
            LayerType::Floor(FloorType::Outdoor) => '·',
            LayerType::Note => 'n',
            LayerType::Structure(StructureType::Boulder) => '*',
            LayerType::Path => '♦',
            LayerType::Item(ItemRarity::Rare) => 'r',
            LayerType::Floor(FloorType::Indoor) => '=',
            LayerType::Structure(StructureType::Wall) => '|',
            LayerType::Structure(StructureType::Rubble) => '&',
            LayerType::Structure(StructureType::Other) => 's',
            LayerType::Structure(StructureType::Table) => '¬',
            LayerType::Structure(StructureType::Rocks) => '.',
            LayerType::Item(ItemRarity::Uncommon) => 'u',
        }
    }
}

impl From<char> for LayerType {
    fn from(c: char) -> Self {
        match c {
            'c' => LayerType::Item(ItemRarity::Common),
            'D' => LayerType::Structure(StructureType::Door),
            '_' => LayerType::Empty,
            'o' => LayerType::Entrance,
            'x' => LayerType::Exit,
            'i' => LayerType::Item(ItemRarity::Exotic),
            '·' => LayerType::Floor(FloorType::Outdoor),
            'n' => LayerType::Note,
            '*' => LayerType::Structure(StructureType::Boulder),
            '♦' => LayerType::Path,
            'r' => LayerType::Item(ItemRarity::Rare),
            '=' => LayerType::Floor(FloorType::Indoor),
            '|' => LayerType::Structure(StructureType::Wall),
            '&' => LayerType::Structure(StructureType::Rubble),
            's' => LayerType::Structure(StructureType::Other),
            '¬' => LayerType::Structure(StructureType::Table),
            'u' => LayerType::Item(ItemRarity::Uncommon),
            '.' => LayerType::Structure(StructureType::Rocks),
            _ => LayerType::Empty,
        }
    }
}

impl LayerType {
    pub fn can_bury_other_layers() -> Vec<LayerType> {
        vec![
            LayerType::Floor(FloorType::Outdoor),
            LayerType::Structure(StructureType::Wall),
            LayerType::Floor(FloorType::Indoor),
        ]
    }

    pub fn is_walkable(&self) -> bool {
        let walkable_types = vec![
            LayerType::Structure(StructureType::Door),
            LayerType::Floor(FloorType::Indoor),
            LayerType::Floor(FloorType::Outdoor),
            LayerType::Structure(StructureType::Rubble),
            LayerType::Item(ItemRarity::Common),
            LayerType::Item(ItemRarity::Uncommon),
            LayerType::Item(ItemRarity::Rare),
            LayerType::Item(ItemRarity::Exotic),
            LayerType::Note,
            LayerType::Path,
            LayerType::Entrance,
            LayerType::Exit,
        ];
        walkable_types.contains(self)
    }

    pub fn is_spawnable(&self) -> bool {
        let spawnable_types = vec![
            LayerType::Floor(FloorType::Indoor),
            LayerType::Floor(FloorType::Outdoor),
        ];
        spawnable_types.contains(self)
    }

    pub fn is_completely_obstructed(&self) -> bool {
        self == &LayerType::Structure(StructureType::Boulder) || self == &LayerType::Empty
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::map::layers::FloorType;

    use super::LayerType;

    #[derive(Deserialize, Serialize)]
    struct Container {
        pub layer_type: LayerType,
    }

    #[test]
    pub fn serialize() {
        let container = Container {
            layer_type: LayerType::Floor(FloorType::Outdoor),
        };

        let serialized = serde_json::to_string(&container).unwrap();

        assert_eq!("{\"layer_type\":{\"floor\":\"Outdoor\"}}", serialized);
    }

    #[test]
    pub fn deserialize() {
        let serialized = "{\"layer_type\":{\"floor\":\"Indoor\"}}";
        let container: Container = serde_json::from_str(serialized).unwrap();
        assert_eq!(LayerType::Floor(FloorType::Indoor), container.layer_type);
    }
}
