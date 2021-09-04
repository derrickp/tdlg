#[derive(PartialEq, Eq, Copy, Clone)]
pub enum CellType {
    Floor,
    RoomFloor,
    OuterWall,
    RoomWall,
    Door,
    None,
}
