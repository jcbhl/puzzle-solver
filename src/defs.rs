#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}
pub struct Position {
    pub center: Point,
    pub orientation: Orientation,
}

pub enum Orientation {
    // viewed from the top
    FlatUp,              // ┴
    FlatLeft,            // -|
    FlatDown,            // T
    FlatRight,           // -|
    StickdownHorizontal, // ---
    StickdownVertical,   // |
    StickupHorizontal,   // ---
    StickupVertical,     // |
    UprightUp,           // |
    UprightLeft,         // --
    UprightDown,         // |
    UprightRight,        // --
}
