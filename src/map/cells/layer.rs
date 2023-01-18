use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LayerType {
    CommonItem,
    Door,
    Empty,
    Entrance,
    Exit,
    ExoticItem,
    Floor,
    Note,
    OuterWall,
    Path,
    RareItem,
    RoomFloor,
    RoomWall,
    Rubble,
    Structure,
    Table,
    UncommonItem,
}

impl Default for LayerType {
    fn default() -> Self {
        Self::Empty
    }
}

impl From<LayerType> for char {
    fn from(val: LayerType) -> Self {
        match val {
            LayerType::CommonItem => 'c',
            LayerType::Door => 'D',
            LayerType::Empty => '_',
            LayerType::Entrance => 'o',
            LayerType::Exit => 'x',
            LayerType::ExoticItem => 'i',
            LayerType::Floor => '·',
            LayerType::Note => 'n',
            LayerType::OuterWall => '*',
            LayerType::Path => '♦',
            LayerType::RareItem => 'r',
            LayerType::RoomFloor => '=',
            LayerType::RoomWall => '|',
            LayerType::Rubble => '&',
            LayerType::Structure => 's',
            LayerType::Table => '¬',
            LayerType::UncommonItem => 'u',
        }
    }
}

impl From<char> for LayerType {
    fn from(c: char) -> Self {
        match c {
            'c' => LayerType::CommonItem,
            'D' => LayerType::Door,
            '_' => LayerType::Empty,
            'o' => LayerType::Entrance,
            'x' => LayerType::Exit,
            'i' => LayerType::ExoticItem,
            '·' => LayerType::Floor,
            'n' => LayerType::Note,
            '*' => LayerType::OuterWall,
            '♦' => LayerType::Path,
            'r' => LayerType::RareItem,
            '=' => LayerType::RoomFloor,
            '|' => LayerType::RoomWall,
            '&' => LayerType::Rubble,
            's' => LayerType::Structure,
            '¬' => LayerType::Table,
            'u' => LayerType::UncommonItem,
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
            LayerType::CommonItem,
            LayerType::UncommonItem,
            LayerType::RareItem,
            LayerType::ExoticItem,
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
