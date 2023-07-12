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

mod defs;
use crate::defs::*;
mod helpers;
use crate::helpers::*;
#[allow(dead_code)]

fn main() {
    let mut board = Board::new();
    solve_level(&mut board);
}

fn solve(mut board: &mut Board) {}

fn solve_level(mut board: &mut Board) {
    while board.cursor != END_OF_BOARD && board.state != State::Done {
        if let Some(p) = try_orientations(&board, board.cursor.clone(), board.state.clone()) {
            if helpers::need_check_overhang(&p.orientation)
                && helpers::has_empty_overhang(&board, &p.center, &p.orientation)
            {
                println!("Bailing out, piece location has overhang.")
            } else {
                println!(
                    "Placing piece at point {:?} with orientation {:?}",
                    p.center, p.orientation
                );
                place_piece_at(&mut board, &p.center, &p.orientation)
            }
        };

        if board.cursor.x == BOARD_SIZE - 1
            && board.cursor.y == BOARD_SIZE - 1
            && board.state != State::Done
        {
            println!("State transition from {:?}", board.state);
            match board.state {
                State::PlacingFlat => board.state = State::PlacingFaceup,
                State::PlacingFaceup => {
                    board.state = State::PlacingFacedown;
                    board.cursor.z += 1;
                }
                State::PlacingFacedown => board.state = State::PlacingUpright,
                State::PlacingUpright => board.state = State::Done,
                State::Done => panic!(),
            }
            board.cursor.x = 0;
            board.cursor.y = 0;
        } else if board.state == State::Done {
            // We've already upped the Z level from the state transition.
            board.cursor.x = 0;
            board.cursor.y = 0;
            board.state = State::PlacingFlat
        } else {
            increment_cursor_in_slice(&mut board.cursor);
        }
    }

    println!(
        "Searched through all of board level {}, did not find any space.",
        board.cursor.z
    );
    // println!("{:?}", board.occupied);
}

fn all_points_clear(board: &Board, points: [Point; 4]) -> bool {
    return points
        .iter()
        .all(|&point| inbounds_and_clear(&board, &point));
}

fn try_orientations(board: &Board, point: Point, state: State) -> Option<Position> {
    println!(
        "Checking orientations for center point {:?} and state {:?}",
        point, state
    );
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
        let points = helpers::get_points_for_orientation(&point, orientation);
        if all_points_clear(&board, points) {
            println!(
                "!!!!!!Found working piece position at {:?} with orientation {:?}",
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
    // println!("New board state is:\n{:?}", board.occupied);
}

fn increment_cursor_in_slice(cursor: &mut Point) {
    cursor.x += 1;
    if cursor.x == BOARD_SIZE {
        cursor.x = 0;
        cursor.y += 1;
    }
}
