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
    solve(&mut board);
}

fn solve(mut board: &mut Board) {
    // Keep track of the state with a stack so that we get DFS.
    // At each step, find the next move given the current PlacementState and the cursor position, then update the board and add the move to the stack.
    // If we don't have any actions, then pop the top of the stack, undo the move, and continue searching from that cursor position and state.
    let mut stack: Vec<StackState> = Vec::new();

    // This should never get popped, only adding so that we have a cursor position on the stack.
    stack.push(StackState {
        placement_state: PlacementState::PlacingFlat,
        last_move: Position {
            center: Point::default(),
            orientation: Orientation::FlatUp,
        },
        cursor: Point::default(),
        pieces_remaining: PIECE_COUNT,
    });

    loop {
        let mut stack_state = stack.last_mut().unwrap();
        // println!("Current stack state is:\n {:?}", stack_state);

        let mut should_pop = false;

        if stack_state.pieces_remaining == 0 {
            println!("---------------------Zero pieces remaining on the stack, solution found. Unwinding...");
            stack
                .iter()
                .for_each(|elem| println!("Place {:?} in orientation {:?}", elem.last_move.center, elem.last_move.orientation));
            return;
        } else if stack_state.cursor.x > BOARD_SIZE - 1 || stack_state.cursor.y > BOARD_SIZE - 1 {
            // println!("Cursor is out of board range, checking...");
            // Bail out if we're PlacingUpright and the slice is not full.
            if stack_state.placement_state == PlacementState::PlacingUpright {
                'outer: for x in 0..BOARD_SIZE {
                    for y in 0..BOARD_SIZE {
                        if !board.occupied[[x, y, stack_state.cursor.z - 1]] {
                            // println!("Found unfilled box at {:?}. Unwinding", [x, y, stack_state.cursor.z - 1]);
                            remove_piece_at(&mut board, &stack_state.last_move.center, &stack_state.last_move.orientation);
                            should_pop = true;
                            break 'outer;
                        }
                    }
                }
            }
            let next_placement_state = placement_state_transition(stack_state.placement_state);
            // println!("State change from {:?} to {:?}", stack_state.placement_state, next_placement_state);

            stack_state.cursor.x = 0;
            stack_state.cursor.y = 0;
            if next_placement_state == PlacementState::PlacingFacedown {
                stack_state.cursor.z += 1;
            }

            stack_state.placement_state = next_placement_state
        } else if let Some(found_position) = try_orientations(&board, &stack_state.cursor, &stack_state.placement_state) {
            if helpers::need_check_overhang(&found_position.orientation)
                && helpers::has_empty_overhang(&board, &found_position.center, &found_position.orientation)
            {
                increment_cursor_in_slice(&mut stack_state.cursor);
                // println!("Bailing out, position has overhang.");
            } else {
                // println!(
                //     "Placing piece at point {:?} with orientation {:?}",
                //     found_position.center, found_position.orientation
                // );
                place_piece_at(&mut board, &found_position.center, &found_position.orientation);
                increment_cursor_in_slice(&mut stack_state.cursor);
                let new_stack_state = StackState {
                    placement_state: stack_state.placement_state,
                    last_move: Position {
                        center: found_position.center,
                        orientation: found_position.orientation,
                    },
                    cursor: stack_state.cursor,
                    pieces_remaining: stack_state.pieces_remaining - 1,
                };
                stack.push(new_stack_state);
            }
        } else {
            // println!("No moves found, incrementing.");
            increment_cursor_in_slice(&mut stack_state.cursor);
        }

        if should_pop {
            stack.pop();
            // increment_cursor_in_slice(&mut stack.last_mut().unwrap().cursor);
        }

        // let mut buf = String::new();
        // let _ = io::stdin().read_line(&mut buf);
    }
}

fn placement_state_transition(state: PlacementState) -> PlacementState {
    return match state {
        PlacementState::PlacingFlat => PlacementState::PlacingFaceup,
        PlacementState::PlacingFaceup => PlacementState::PlacingFacedown,
        PlacementState::PlacingFacedown => PlacementState::PlacingUpright,
        PlacementState::PlacingUpright => PlacementState::PlacingFlat,
    };
}

// fn solve_level(mut board: &mut Board) {
//     while board.cursor != END_OF_BOARD && board.state != PlacementState::Done {
//         if let Some(p) = try_orientations(&board, &board.cursor, &board.state) {
//             if helpers::need_check_overhang(&p.orientation) && helpers::has_empty_overhang(&board, &p.center, &p.orientation) {
//                 println!("Bailing out, piece location has overhang.")
//             } else {
//                 println!("Placing piece at point {:?} with orientation {:?}", p.center, p.orientation);
//                 place_piece_at(&mut board, &p.center, &p.orientation)
//             }
//         };

//         if board.cursor.x == BOARD_SIZE - 1 && board.cursor.y == BOARD_SIZE - 1 && board.state != PlacementState::Done {
//             println!("State transition from {:?}", board.state);
//             board.cursor.x = 0;
//             board.cursor.y = 0;
//         } else if board.state == PlacementState::Done {
//             // We've already upped the Z level from the state transition.
//             board.cursor.x = 0;
//             board.cursor.y = 0;
//             board.state = PlacementState::PlacingFlat
//         } else {
//             increment_cursor_in_slice(&mut board.cursor);
//         }
//     }

//     println!("Searched through all of board level {}, did not find any space.", board.cursor.z);
//     // println!("{:?}", board.occupied);
// }

fn all_points_clear(board: &Board, points: [Point; 4]) -> bool {
    return points.iter().all(|&point| inbounds_and_clear(&board, &point));
}

fn try_orientations(board: &Board, point: &Point, state: &PlacementState) -> Option<Position> {
    // println!("Checking orientations for center point {:?} and state {:?}", point, state);
    if !inbounds_and_clear(&board, &point) {
        return None;
    }
    let mut orientations: Vec<Orientation> = Default::default();
    match state {
        PlacementState::PlacingFlat => {
            orientations.push(Orientation::FlatUp);
            orientations.push(Orientation::FlatLeft);
            orientations.push(Orientation::FlatDown);
            orientations.push(Orientation::FlatRight);
        }
        PlacementState::PlacingFaceup => {
            orientations.push(Orientation::FaceupHorizontal);
            orientations.push(Orientation::FaceupVertical);
        }
        PlacementState::PlacingFacedown => {
            orientations.push(Orientation::FacedownHorizontal);
            orientations.push(Orientation::FacedownVertical);
        }
        PlacementState::PlacingUpright => {
            orientations.push(Orientation::UprightUp);
            orientations.push(Orientation::UprightLeft);
            orientations.push(Orientation::UprightDown);
            orientations.push(Orientation::UprightRight);
        }
    }

    for orientation in orientations {
        let points = helpers::get_points_for_orientation(&point, &orientation);
        if all_points_clear(&board, points) {
            // println!("!!!!!!Found working piece position at {:?} with orientation {:?}", point, orientation);
            return Some(Position {
                center: *point,
                orientation: orientation,
            });
        }
    }

    None
}

fn place_piece_at(board: &mut Board, point: &Point, orientation: &Orientation) {
    let points = get_points_for_orientation(point, orientation);
    for target_point in points {
        assert!(!board.occupied[[target_point.x, target_point.y, target_point.z]]);
        board.occupied[[target_point.x, target_point.y, target_point.z]] = true;
    }
    // println!("Successfully placed piece at {:?} with orientation {:?}", point, orientation);
}

fn remove_piece_at(board: &mut Board, point: &Point, orientation: &Orientation) {
    let points = get_points_for_orientation(point, orientation);
    for target_point in points {
        assert!(board.occupied[[target_point.x, target_point.y, target_point.z]]);
        board.occupied[[target_point.x, target_point.y, target_point.z]] = false;
    }
    // println!("Successfully removed piece at {:?} with orientation {:?}", point, orientation);
    // println!("New board state is:\n{:?}", board.occupied);
}

fn increment_cursor_in_slice(cursor: &mut Point) {
    cursor.x += 1;
    if cursor.x == BOARD_SIZE {
        cursor.x = 0;
        cursor.y += 1;
    }
}
