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
}

impl LayerType {
    pub fn is_walkable(&self) -> bool {
        self == &LayerType::Door
            || self == &LayerType::RoomFloor
            || self == &LayerType::Floor
            || self == &LayerType::Rubble
    }

    pub fn is_spawnable(&self) -> bool {
        self == &LayerType::RoomFloor || self == &LayerType::Floor
    }
}
