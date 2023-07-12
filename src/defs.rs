#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
