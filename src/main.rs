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
enum State {
    PlacingFlat,
    PlacingFaceup,
    PlacingFacedown,
    PlacingUpright,
}
struct Board {
    occupied: Array3<bool>,
    current_unfilled_layer: usize,
    cursor: Point,
    state: State,
}

impl Board {
    fn new() -> Self {
        Self {
            occupied: Array3::default((BOARD_SIZE, BOARD_SIZE, BOARD_SIZE)),
            current_unfilled_layer: 0,
            cursor: Point { x: 0, y: 0, z: 0 },
            state: State::PlacingFlat,
        }
    }
    fn at(&self, point: Point) -> bool {
        return self.occupied[[point.x, point.y, point.z]];
    }
}

fn main() {
    let mut board = Board::new();
    // solve(&mut board);
}

fn solve(board: &mut Board) {
    // DFS on the board
}

fn get_next_placement(board: &mut Board) -> Option<Position> {
    let ending_point = Point {
        x: BOARD_SIZE - 1,
        y: BOARD_SIZE - 1,
        z: board.current_unfilled_layer,
    };

    while board.cursor != ending_point {
        // if let try_orient
        // increment_cursor_in_slice(&mut board.cursor)
    }

    println!(
        "Searched through all of board level {}, did not find any space.",
        board.current_unfilled_layer
    );
    println!("{:?}", board.occupied);
    None
}

fn inbounds_and_clear(board: &Board, point: &Point) -> bool {
    return point.x <= BOARD_SIZE - 1 && point.y <= BOARD_SIZE - 1 && point.z <= BOARD_SIZE - 1;
}
fn try_orientations(board: &Board, point: &Point, state: State) -> Option<Position> {
    match state {
        State::PlacingFlat => {
            // FlatUp
            if inbounds_and_clear(
                &board,
                &Point {
                    x: point.x,
                    y: point.y,
                    z: point.z,
                },
            ) && inbounds_and_clear(
                &board,
                &Point {
                    x: point.x.wrapping_sub(1),
                    y: point.y,
                    z: point.z,
                },
            ) && inbounds_and_clear(
                &board,
                &Point {
                    x: point.x + 1,
                    y: point.y,
                    z: point.z,
                },
            ) && inbounds_and_clear(
                &board,
                &Point {
                    x: point.x,
                    y: point.y.wrapping_sub(1),
                    z: point.z,
                },
            ) {
                return Some(Position {
                    center: point.clone(),
                    orientation: Orientation::FlatUp,
                });
            }
            // FlatLeft
            // FlatDown
            // FlatRight
            None
        }
        State::PlacingFaceup => todo!(),
        State::PlacingFacedown => todo!(),
        State::PlacingUpright => todo!(),
    }
}

fn increment_cursor_in_slice(cursor: &mut Point) {
    cursor.x += 1;
    if cursor.x == BOARD_SIZE {
        cursor.x = 0;
        cursor.y += 1;
    }
}
