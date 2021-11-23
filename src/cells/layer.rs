#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum LayerType {
    Door,
    Empty,
    Floor,
    OuterWall,
    RoomFloor,
    RoomWall,
    Rubble,
    Table,
    Structure,
    CommonItem,
    UncommonItem,
    RareItem,
    ExoticItem,
    Note,
}

impl From<char> for LayerType {
    fn from(c: char) -> Self {
        match c {
            'w' => LayerType::RoomWall,
            'f' => LayerType::RoomFloor,
            'd' => LayerType::Door,
            'r' => LayerType::Rubble,
            't' => LayerType::Table,
            'l' => LayerType::Floor,
            'e' => LayerType::Empty,
            'o' => LayerType::OuterWall,
            's' => LayerType::Structure,
            'c' => LayerType::CommonItem,
            'u' => LayerType::UncommonItem,
            'a' => LayerType::RareItem,
            'x' => LayerType::ExoticItem,
            'n' => LayerType::Note,
            _ => LayerType::Empty,
        }
    }
}

impl LayerType {
    pub fn is_walkable(&self) -> bool {
        let walkable_types = vec![
            LayerType::Door,
            LayerType::RoomFloor,
            LayerType::Floor,
            LayerType::RoomFloor,
        ];
        walkable_types.contains(self)
    }

    pub fn is_spawnable(&self) -> bool {
        let spawnable_types = vec![LayerType::RoomFloor, LayerType::Floor];
        spawnable_types.contains(self)
    }
}
