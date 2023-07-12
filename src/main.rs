/*
    z
    |
    |
    |
    |
    |________________ x
   /
  /
 /
/
y

*/

const BOARD_SIZE: usize = 6;
use ndarray::Array3;

mod defs;
use crate::defs::*;
#[allow(dead_code)]
#[derive(Debug)]
enum State{
    PlacingFlat,
    PlacingFaceup,
    PlacingFacedown,
    PlacingUpright
}
struct Board {
    occupied: Array3<bool>,
    current_unfilled_layer: usize,
    cursor: Point,
    state: State
}

impl Board {
    fn new() -> Self {
        Self {
            occupied: Array3::default((BOARD_SIZE, BOARD_SIZE, BOARD_SIZE)),
            current_unfilled_layer: 0,
            cursor: Point{x:0, y:0, z:0},
            state: PlacingFlat
        }
    }
    fn at(&self, point: Point) -> bool{
        return self.occupied[[point.x, point.y, point.z]];
    }
}

fn main() {
    let mut board = Board::new();
    solve(&mut board);
}

fn solve(board: &mut Board) {
    // DFS on the board
}

fn get_next_placement(board: &mut Board) -> Option<Position> {
    let ending_
    while board.cursor != Point{x: BOARD_SIZE - 1, y:BOARD_SIZE - 1, z:board.current_unfilled_layer}) {

        increment_cursor_in_slice(&mut board.cursor)
    }

    println!(
        "Searched through all of board level {}, did not find any space.",
        board.current_unfilled_layer
    );
    println!("{:?}", board.occupied);

    None
}


fn increment_cursor_in_slice(cursor: &mut (usize, usize)) {
    cursor.0 += 1;
    if cursor.0 == BOARD_SIZE {
        cursor.0 = 0;
        cursor.1 += 1;
    }
}
