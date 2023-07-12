use ndarray::Array3;

pub const BOARD_SIZE: usize = 6;
pub const END_OF_BOARD: Point = Point {
    x: BOARD_SIZE - 1,
    y: BOARD_SIZE - 1,
    z: BOARD_SIZE - 1,
};

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
    FlatRight,          // -|
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
pub enum State {
    PlacingFlat,
    PlacingFaceup,
    PlacingFacedown,
    PlacingUpright,
    Done,
}
pub struct Board {
    pub occupied: Array3<bool>,
    pub cursor: Point,
    pub state: State,
}

impl Board {
    pub fn new() -> Self {
        Self {
            occupied: Array3::default((BOARD_SIZE, BOARD_SIZE, BOARD_SIZE)),
            cursor: Point { x: 0, y: 0, z: 0 },
            state: State::PlacingFlat,
        }
    }
}
