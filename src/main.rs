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
#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    PlacingFlat,
    PlacingFaceup,
    PlacingFacedown,
    PlacingUpright,
    Done,
}
struct Board {
    occupied: Array3<bool>,
    cursor: Point,
    state: State,
}

impl Board {
    fn new() -> Self {
        Self {
            occupied: Array3::default((BOARD_SIZE, BOARD_SIZE, BOARD_SIZE)),
            cursor: Point { x: 0, y: 0, z: 0 },
            state: State::PlacingFlat,
        }
    }
}

fn main() {
    let mut board = Board::new();
    solve_level(&mut board);
}

fn solve_level(mut board: &mut Board) {
    let ending_point = Point {
        x: BOARD_SIZE - 1,
        y: BOARD_SIZE - 1,
        z: BOARD_SIZE - 1,
    };

    while board.cursor != ending_point && board.state != State::PlacingUpright {
        let last_in_level = Point {
            x: BOARD_SIZE - 1,
            y: BOARD_SIZE - 1,
            z: board.cursor.z,
        };

        if let Some(p) = try_orientations(&board, board.cursor.clone(), board.state.clone()) {
            println!(
                "Placing piece at point {:?} with orientation {:?}",
                p.center, p.orientation
            );
            place_piece_at(&mut board, &p.center, &p.orientation)
        };

        if board.cursor == last_in_level && board.state != State::Done {
            match board.state {
                State::PlacingFlat => board.state = State::PlacingFaceup,
                State::PlacingFaceup => board.state = State::PlacingFacedown,
                State::PlacingFacedown => board.state = State::PlacingUpright,
                State::PlacingUpright => board.state = State::Done,
                State::Done => panic!(),
            }
        } else if board.state == State::Done {
            board.cursor = Point {
                x: 0,
                y: 0,
                z: board.cursor.z + 1,
            };
            board.state = State::PlacingFlat
        } else {
            increment_cursor_in_slice(&mut board.cursor);
        }
    }

    println!(
        "Searched through all of board level {}, did not find any space.",
        board.cursor.z
    );
    println!("{:?}", board.occupied);
}

fn inbounds_and_clear(board: &Board, point: &Point) -> bool {
    return point.x <= BOARD_SIZE - 1
        && point.y <= BOARD_SIZE - 1
        && point.z <= BOARD_SIZE - 1
        && !board.occupied[[point.x, point.y, point.z]];
}

fn all_points_clear(board: &Board, points: [Point; 4]) -> bool {
    return points
        .iter()
        .all(|&point| inbounds_and_clear(&board, &point));
}

fn try_orientations(board: &Board, point: Point, state: State) -> Option<Position> {
    println!("Checking orientations for center point {:?}", point);
    if !inbounds_and_clear(&board, &point) {
        return None;
    }
    let mut orientations: Vec<Orientation> = Default::default();
    match state {
        State::PlacingFlat => {
            orientations.push(Orientation::FlatUp);
            orientations.push(Orientation::FlatLeft);
            orientations.push(Orientation::FlatDown);
            orientations.push(Orientation::FlatRight);
        }
        State::PlacingFaceup => {
            orientations.push(Orientation::FaceupHorizontal);
            orientations.push(Orientation::FaceupVertical);
        }
        State::PlacingFacedown => {
            orientations.push(Orientation::FacedownHorizontal);
            orientations.push(Orientation::FacedownVertical);
        }
        State::PlacingUpright => {
            orientations.push(Orientation::UprightUp);
            orientations.push(Orientation::UprightLeft);
            orientations.push(Orientation::UprightDown);
            orientations.push(Orientation::UprightRight);
        }
        State::Done => panic!(),
    }

    for orientation in orientations {
        let points = get_points_for_orientation(&point, orientation);
        if all_points_clear(&board, points) {
            println!(
                "Found working piece position at {:?} with orientation {:?}",
                point, orientation
            );
            return Some(Position {
                center: point.clone(),
                orientation: orientation.clone(),
            });
        }
    }

    None
}

fn place_piece_at(board: &mut Board, point: &Point, orientation: &Orientation) {
    let points = get_points_for_orientation(point, orientation.clone());
    for target_point in points {
        assert!(!board.occupied[[target_point.x, target_point.y, target_point.z]]);
        board.occupied[[target_point.x, target_point.y, target_point.z]] = true;
    }
    println!(
        "Successfully placed piece at {:?} with orientation {:?}",
        point, orientation
    );
}

fn get_points_for_orientation(point: &Point, orientation: Orientation) -> [Point; 4] {
    let mut points: [Point; 4] = Default::default();
    points[0] = point.clone();

    match orientation {
        Orientation::FlatUp => {
            points[1] = Point {
                x: point.x.wrapping_sub(1),
                y: point.y,
                z: point.z,
            };
            points[2] = Point {
                x: point.x + 1,
                y: point.y,
                z: point.z,
            };
            points[3] = Point {
                x: point.x,
                y: point.y.wrapping_sub(1),
                z: point.z,
            };
        }
        Orientation::FlatLeft => {
            points[1] = Point {
                x: point.x.wrapping_sub(1),
                y: point.y,
                z: point.z,
            };
            points[2] = Point {
                x: point.x,
                y: point.y.wrapping_sub(1),
                z: point.z,
            };
            points[3] = Point {
                x: point.x,
                y: point.y + 1,
                z: point.z,
            };
        }
        Orientation::FlatDown => {
            points[1] = Point {
                x: point.x.wrapping_sub(1),
                y: point.y,
                z: point.z,
            };
            points[2] = Point {
                x: point.x + 1,
                y: point.y,
                z: point.z,
            };
            points[3] = Point {
                x: point.x,
                y: point.y + 1,
                z: point.z,
            };
        }
        Orientation::FlatRight => {
            points[1] = Point {
                x: point.x,
                y: point.y.wrapping_sub(1),
                z: point.z,
            };
            points[2] = Point {
                x: point.x,
                y: point.y + 1,
                z: point.z,
            };
            points[3] = Point {
                x: point.x + 1,
                y: point.y,
                z: point.z,
            };
        }
        Orientation::FacedownHorizontal => {
            points[1] = Point {
                x: point.x.wrapping_sub(1),
                y: point.y,
                z: point.z,
            };
            points[2] = Point {
                x: point.x + 1,
                y: point.y,
                z: point.z,
            };
            points[3] = Point {
                x: point.x,
                y: point.y,
                z: point.z - 1,
            };
        }
        Orientation::FacedownVertical => {
            points[1] = Point {
                x: point.x,
                y: point.y.wrapping_sub(1),
                z: point.z,
            };
            points[2] = Point {
                x: point.x,
                y: point.y + 1,
                z: point.z,
            };
            points[3] = Point {
                x: point.x,
                y: point.y,
                z: point.z + 1,
            };
        }
        Orientation::FaceupHorizontal => {
            points[1] = Point {
                x: point.x.wrapping_sub(1),
                y: point.y,
                z: point.z,
            };
            points[2] = Point {
                x: point.x + 1,
                y: point.y,
                z: point.z,
            };
            points[3] = Point {
                x: point.x,
                y: point.y,
                z: point.z + 1,
            };
        }
        Orientation::FaceupVertical => {
            points[1] = Point {
                x: point.x,
                y: point.y.wrapping_sub(1),
                z: point.z,
            };
            points[2] = Point {
                x: point.x,
                y: point.y + 1,
                z: point.z,
            };
            points[3] = Point {
                x: point.x,
                y: point.y,
                z: point.z + 1,
            };
        }
        Orientation::UprightUp => {
            points[1] = Point {
                x: point.x,
                y: point.y,
                z: point.z + 1,
            };
            points[2] = Point {
                x: point.x,
                y: point.y,
                z: point.z.wrapping_sub(1),
            };
            points[3] = Point {
                x: point.x,
                y: point.y.wrapping_sub(1),
                z: point.z,
            };
        }
        Orientation::UprightLeft => {
            points[1] = Point {
                x: point.x,
                y: point.y,
                z: point.z + 1,
            };
            points[2] = Point {
                x: point.x,
                y: point.y,
                z: point.z.wrapping_sub(1),
            };
            points[3] = Point {
                x: point.x.wrapping_sub(1),
                y: point.y,
                z: point.z,
            };
        }
        Orientation::UprightDown => {
            points[1] = Point {
                x: point.x,
                y: point.y,
                z: point.z + 1,
            };
            points[2] = Point {
                x: point.x,
                y: point.y,
                z: point.z.wrapping_sub(1),
            };
            points[3] = Point {
                x: point.x,
                y: point.y + 1,
                z: point.z,
            };
        }
        Orientation::UprightRight => {
            points[1] = Point {
                x: point.x,
                y: point.y,
                z: point.z + 1,
            };
            points[2] = Point {
                x: point.x,
                y: point.y,
                z: point.z.wrapping_sub(1),
            };
            points[3] = Point {
                x: point.x + 1,
                y: point.y,
                z: point.z,
            };
        }
    }
    return points;
}

fn increment_cursor_in_slice(cursor: &mut Point) {
    cursor.x += 1;
    if cursor.x == BOARD_SIZE {
        cursor.x = 0;
        cursor.y += 1;
    }
}
