pub const BOARD_SIZE: usize = 6;
pub const PIECE_COUNT: usize = 54;
pub const THREAD_COUNT: usize = 8;

pub type Solution = Vec<Position>;
pub type Board = ndarray::Array3<bool>;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub center: Point,
    pub orientation: Orientation,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Orientation {
    // viewed from the top
    FlatUp,             // ┴
    FlatLeft,           // -|
    FlatDown,           // T
    FlatRight,          // |-
    FacedownHorizontal, // ---
    FacedownVertical,   // |
    FaceupHorizontal,   // ---
    FaceupVertical,     // |
    UprightUp,          // |
    UprightLeft,        // --
    UprightDown,        // |
    UprightRight,       // --
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum PlacementState {
    PlacingFlat,
    PlacingFaceup,
    PlacingFacedown,
    PlacingUpright,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct StackState {
    pub placement_state: PlacementState,
    pub last_move: Position,
    pub cursor: Point,
    pub pieces_remaining: usize,
}
