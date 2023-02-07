use serde::{Deserialize, Serialize};

use super::item_rarity::ItemRarity;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LayerType {
    Door,
    Empty,
    Entrance,
    Exit,
    Item(ItemRarity),
    Floor,
    Note,
    OuterWall,
    Path,
    RoomFloor,
    RoomWall,
    Rubble,
    Structure,
    Table,
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
            LayerType::Door => 'D',
            LayerType::Empty => '_',
            LayerType::Entrance => 'o',
            LayerType::Exit => 'x',
            LayerType::Item(ItemRarity::Exotic) => 'i',
            LayerType::Floor => '·',
            LayerType::Note => 'n',
            LayerType::OuterWall => '*',
            LayerType::Path => '♦',
            LayerType::Item(ItemRarity::Rare) => 'r',
            LayerType::RoomFloor => '=',
            LayerType::RoomWall => '|',
            LayerType::Rubble => '&',
            LayerType::Structure => 's',
            LayerType::Table => '¬',
            LayerType::Item(ItemRarity::Uncommon) => 'u',
        }
    }
}

impl From<char> for LayerType {
    fn from(c: char) -> Self {
        match c {
            'c' => LayerType::Item(ItemRarity::Common),
            'D' => LayerType::Door,
            '_' => LayerType::Empty,
            'o' => LayerType::Entrance,
            'x' => LayerType::Exit,
            'i' => LayerType::Item(ItemRarity::Exotic),
            '·' => LayerType::Floor,
            'n' => LayerType::Note,
            '*' => LayerType::OuterWall,
            '♦' => LayerType::Path,
            'r' => LayerType::Item(ItemRarity::Rare),
            '=' => LayerType::RoomFloor,
            '|' => LayerType::RoomWall,
            '&' => LayerType::Rubble,
            's' => LayerType::Structure,
            '¬' => LayerType::Table,
            'u' => LayerType::Item(ItemRarity::Uncommon),
            _ => LayerType::Empty,
        }
    }
}

impl LayerType {
    pub fn can_bury_other_layers() -> Vec<LayerType> {
        vec![LayerType::Floor, LayerType::RoomWall, LayerType::RoomFloor]
    }

    pub fn is_walkable(&self) -> bool {
        let walkable_types = vec![
            LayerType::Door,
            LayerType::RoomFloor,
            LayerType::Floor,
            LayerType::RoomFloor,
            LayerType::Rubble,
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
        let spawnable_types = vec![LayerType::RoomFloor, LayerType::Floor];
        spawnable_types.contains(self)
    }

    pub fn is_completely_obstructed(&self) -> bool {
        self == &LayerType::OuterWall || self == &LayerType::Empty
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::LayerType;

    #[derive(Deserialize, Serialize)]
    struct Container {
        pub layer_type: LayerType,
    }

    #[test]
    pub fn serialize() {
        let container = Container {
            layer_type: LayerType::Floor,
        };

        let serialized = serde_json::to_string(&container).unwrap();

        assert_eq!("{\"layer_type\":\"floor\"}", serialized);
    }

    #[test]
    pub fn deserialize() {
        let serialized = "{\"layer_type\":\"room_floor\"}";
        let container: Container = serde_json::from_str(serialized).unwrap();
        assert_eq!(LayerType::RoomFloor, container.layer_type);
    }
}
