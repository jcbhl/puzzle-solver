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

use core::time;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;

fn main() {
    let search_results: Arc<Mutex<Vec<Solution>>> = Default::default();

    for thread_num in 0..THREAD_COUNT {
        let initial_stack_state = StackState {
            placement_state: PlacementState::PlacingFlat,
            last_move: Position {
                center: Point::default(),
                orientation: Orientation::FlatUp,
            },
            cursor: get_cursor_from_thread_num(thread_num),
            pieces_remaining: PIECE_COUNT,
        };
        let search_results = search_results.clone();

        thread::spawn(move || {
            println!("Spawned thread {:?}", thread::current().id());
            solve(search_results, initial_stack_state);
        });
    }

    loop {
        {
            let mut vec = search_results.lock().unwrap();

            if !vec.is_empty() {
                println!("Found new solution:");
                vec.pop()
                    .unwrap()
                    .iter()
                    .for_each(|position| println!("Place {:?} in orientation {:?}", position.center, position.orientation));
                println!("\n\n");
            }
        }

        sleep(time::Duration::from_millis(100));
    }
}

// Gives each thread a point on the first layer to start the search at.
fn get_cursor_from_thread_num(thread_id: usize) -> Point {
    let starting_point_fraction = thread_id as f64 / THREAD_COUNT as f64;
    let total_squares = BOARD_SIZE * BOARD_SIZE;
    let starting_square_idx = (starting_point_fraction * total_squares as f64) as usize;
    let y = starting_square_idx / BOARD_SIZE;
    let x = starting_square_idx % BOARD_SIZE;
    Point { x, y, z: 0 }
}

fn solve(results: Arc<Mutex<Vec<Solution>>>, initial_stack_state: StackState) {
    // Core solver loop - DFS with pruning.
    // At each step, find the next move given the current PlacementState and the cursor position, then update the board and add the move to the stack.
    // If we don't have any possible actions, then pop the top of the stack, undo the move, and continue searching from that cursor position and state.
    let mut stack: Vec<StackState> = Vec::new();

    let mut board = Board::new();

    // This should never get popped, only adding so that we have a cursor position on the stack.
    stack.push(initial_stack_state);

    loop {
        let mut stack_state = stack.last_mut().unwrap();
        let mut should_pop = false;

        // We've found a valid solution, pop it off the stack and return the result to the main thread.
        if stack_state.pieces_remaining == 0 {
            let this_result: Vec<Position> = stack.iter().map(|stackstate| stackstate.last_move).collect();
            {
                results.lock().unwrap().push(this_result);
            }
            // TODO continue searching
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
        }
    }
}

fn placement_state_transition(state: PlacementState) -> PlacementState {
    match state {
        PlacementState::PlacingFlat => PlacementState::PlacingFaceup,
        PlacementState::PlacingFaceup => PlacementState::PlacingFacedown,
        PlacementState::PlacingFacedown => PlacementState::PlacingUpright,
        PlacementState::PlacingUpright => PlacementState::PlacingFlat,
    }
}

fn all_points_clear(board: &Board, points: [Point; 4]) -> bool {
    // This originally was written as an iter-all expression, but rustc couldn't unroll the iter
    // even though the array length is known at compile time so this is much faster.
    for point in points {
        if !inbounds_and_clear(board, &point) {
            return false;
        }
    }
    true
}

fn try_orientations(board: &Board, point: &Point, state: &PlacementState) -> Option<Position> {
    // println!("Checking orientations for center point {:?} and state {:?}", point, state);
    if !inbounds_and_clear(board, point) {
        return None;
    }
    let orientations: [Option<Orientation>; 4] = match state {
        PlacementState::PlacingFlat => [
            Some(Orientation::FlatUp),
            Some(Orientation::FlatLeft),
            Some(Orientation::FlatDown),
            Some(Orientation::FlatRight),
        ],
        PlacementState::PlacingFaceup => [Some(Orientation::FaceupHorizontal), Some(Orientation::FaceupVertical), None, None],
        PlacementState::PlacingFacedown => [
            Some(Orientation::FacedownHorizontal),
            Some(Orientation::FacedownVertical),
            None,
            None,
        ],
        PlacementState::PlacingUpright => [
            Some(Orientation::UprightUp),
            Some(Orientation::UprightLeft),
            Some(Orientation::UprightDown),
            Some(Orientation::UprightRight),
        ],
    };

    for orientation in orientations {
        if orientation.is_none() {
            continue;
        }
        let points = helpers::get_points_for_orientation(point, &orientation.unwrap());
        if all_points_clear(board, points) {
            // println!("!!!!!!Found working piece position at {:?} with orientation {:?}", point, orientation);
            return Some(Position {
                center: *point,
                orientation: orientation.unwrap(),
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
