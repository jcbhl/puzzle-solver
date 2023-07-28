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


use wasm_bindgen::prelude::*;
// use web_sys::console;

use hsl::HSL;
use num_complex::Complex64;
use rand::Rng;
use wasm_bindgen::{prelude::*, Clamped};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

type RGBA = [u8; 4];

struct Generator {
    width: u32,
    height: u32,
    palette: Box<[RGBA]>,
}

impl Generator {
    fn new(width: u32, height: u32, max_iterations: u32) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            width,
            height,
            palette: (0..max_iterations)
                .map(move |_| {
                    let (r, g, b) = HSL {
                        h: rng.gen_range(0.0..360.0),
                        s: 0.5,
                        l: 0.6,
                    }
                    .to_rgb();
                    [r, g, b, 255]
                })
                .collect(),
        }
    }

    #[allow(clippy::many_single_char_names)]
    fn get_color(&self, x: u32, y: u32) -> &RGBA {
        let c = Complex64::new(
            (f64::from(x) - f64::from(self.width) / 2.0) * 4.0 / f64::from(self.width),
            (f64::from(y) - f64::from(self.height) / 2.0) * 4.0 / f64::from(self.height),
        );
        let mut z = Complex64::new(0.0, 0.0);
        let mut i = 0;
        while z.norm_sqr() < 4.0 {
            if i == self.palette.len() {
                return &self.palette[0];
            }
            z = z.powi(2) + c;
            i += 1;
        }
        &self.palette[i]
    }

    fn iter_row_bytes(&self, y: u32) -> impl '_ + Iterator<Item = u8> {
        (0..self.width)
            .flat_map(move |x| self.get_color(x, y))
            .copied()
    }

    // Multi-threaded implementation.
    #[cfg(feature = "rayon")]
    fn iter_bytes(&self) -> impl '_ + ParallelIterator<Item = u8> {
        (0..self.height)
            .into_par_iter()
            .flat_map_iter(move |y| self.iter_row_bytes(y))
    }

    // Single-threaded implementation.
    #[cfg(not(feature = "rayon"))]
    fn iter_bytes(&self) -> impl '_ + Iterator<Item = u8> {
        (0..self.height).flat_map(move |y| self.iter_row_bytes(y))
    }
}

#[wasm_bindgen]
pub fn generate(width: u32, height: u32, max_iterations: u32) -> Clamped<Vec<u8>> {
    Clamped(
        Generator::new(width, height, max_iterations)
            .iter_bytes()
            .collect(),
    )
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue>{
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // console::log_1(&JsValue::from_str("123123123123"));

    Ok(())
}


#[allow(dead_code)]
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

        thread::sleep(time::Duration::from_millis(100));
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

    let mut board = ndarray::Array3::default((BOARD_SIZE, BOARD_SIZE, BOARD_SIZE));

    // This should never get popped, only adding so that we have a cursor position on the stack.
    stack.push(initial_stack_state);

    loop {
        let mut stack_state = stack.last_mut().unwrap();
        let stack_ref = *stack_state;
        let mut should_pop = false;

        // We've found a valid solution, pop it off the stack and return the result to the main thread.
        if stack_state.pieces_remaining == 0 {
            // The first element is going to be the initial cursor position, so ignore it when reporting.
            let this_result: Vec<Position> = stack.iter().skip(1).map(|stackstate| stackstate.last_move).collect();
            {
                results.lock().unwrap().push(this_result);
            }

            // After reporting the result, continue searching.
            remove_piece_at(&mut board, &stack_ref.last_move.center, &stack_ref.last_move.orientation);
            should_pop = true;
        } else if stack_state.cursor.x > BOARD_SIZE - 1 || stack_state.cursor.y > BOARD_SIZE - 1 {
            // Recall that our state transition graph for placement is PlacingFlat => PlacingFaceup
            // => PlacingFacedown => PlacingUpright. Therefore, if we're in PlacingUpright and the board is not yet full,
            // then there's not a valid solution and we should backtrack to the last move.
            if stack_state.placement_state == PlacementState::PlacingUpright {
                'outer: for x in 0..BOARD_SIZE {
                    for y in 0..BOARD_SIZE {
                        if !board[[x, y, stack_state.cursor.z - 1]] {
                            remove_piece_at(&mut board, &stack_state.last_move.center, &stack_state.last_move.orientation);
                            should_pop = true;
                            break 'outer;
                        }
                    }
                }
            }
            let next_placement_state = placement_state_transition(stack_state.placement_state);

            stack_state.cursor.x = 0;
            stack_state.cursor.y = 0;
            if next_placement_state == PlacementState::PlacingFacedown {
                stack_state.cursor.z += 1;
            }

            stack_state.placement_state = next_placement_state
        } else if let Some(found_position) = try_orientations(&board, &stack_state.cursor, &stack_state.placement_state) {
            // Given our current placement state, if there are any valid orientations, check its validity and place it.

            // We don't want to place a piece that will cover any piece on the layer below it.
            if helpers::need_check_overhang(&found_position.orientation)
                && helpers::has_empty_overhang(&board, &found_position.center, &found_position.orientation)
            {
                increment_cursor(&mut stack_state.cursor);
            } else {
                // Place the piece and add the move to the stack.
                place_piece_at(&mut board, &found_position.center, &found_position.orientation);
                increment_cursor(&mut stack_state.cursor);
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
            // No valid moves for this position and orientation, continue on.
            increment_cursor(&mut stack_state.cursor);
        }

        if should_pop {
            assert!(stack.pop().is_some());
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
        #[rustfmt::skip]
        PlacementState::PlacingFaceup => [
            Some(Orientation::FaceupHorizontal), 
            Some(Orientation::FaceupVertical), 
            None, 
            None
        ],
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
        assert!(!board[[target_point.x, target_point.y, target_point.z]]);
        board[[target_point.x, target_point.y, target_point.z]] = true;
    }
}

fn remove_piece_at(board: &mut Board, point: &Point, orientation: &Orientation) {
    let points = get_points_for_orientation(point, orientation);
    for target_point in points {
        assert!(board[[target_point.x, target_point.y, target_point.z]]);
        board[[target_point.x, target_point.y, target_point.z]] = false;
    }
}

fn increment_cursor(cursor: &mut Point) {
    cursor.x += 1;
    if cursor.x == BOARD_SIZE {
        cursor.x = 0;
        cursor.y += 1;
    }
}
