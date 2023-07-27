use crate::defs::*;

// Checks to see if a given point and orientation overhangs an empty point. If so, we don't want to place a piece there.
pub fn has_empty_overhang(board: &Board, point: &Point, orientation: &Orientation) -> bool {
    match orientation {
        Orientation::FacedownHorizontal => {
            inbounds_and_clear(
                board,
                &Point {
                    x: point.x.wrapping_sub(1),
                    y: point.y,
                    z: point.z.wrapping_sub(1),
                },
            ) || inbounds_and_clear(
                board,
                &Point {
                    x: point.x + 1,
                    y: point.y,
                    z: point.z.wrapping_sub(1),
                },
            )
        }
        Orientation::FacedownVertical => {
            inbounds_and_clear(
                board,
                &Point {
                    x: point.x,
                    y: point.y.wrapping_sub(1),
                    z: point.z.wrapping_sub(1),
                },
            ) || inbounds_and_clear(
                board,
                &Point {
                    x: point.x,
                    y: point.y + 1,
                    z: point.z.wrapping_sub(1),
                },
            )
        }
        Orientation::UprightUp => inbounds_and_clear(
            board,
            &Point {
                x: point.x,
                y: point.y.wrapping_sub(1),
                z: point.z.wrapping_sub(1),
            },
        ),
        Orientation::UprightLeft => inbounds_and_clear(
            board,
            &Point {
                x: point.x.wrapping_sub(1),
                y: point.y,
                z: point.z.wrapping_sub(1),
            },
        ),
        Orientation::UprightDown => inbounds_and_clear(
            board,
            &Point {
                x: point.x,
                y: point.y + 1,
                z: point.z.wrapping_sub(1),
            },
        ),
        Orientation::UprightRight => inbounds_and_clear(
            board,
            &Point {
                x: point.x + 1,
                y: point.y,
                z: point.z.wrapping_sub(1),
            },
        ),
        _ => panic!(),
    }
}

pub fn need_check_overhang(orientation: &Orientation) -> bool {
    matches!(
        orientation,
        Orientation::FacedownHorizontal
            | Orientation::FacedownVertical
            | Orientation::UprightUp
            | Orientation::UprightLeft
            | Orientation::UprightDown
            | Orientation::UprightRight
    )
}

pub fn inbounds_and_clear(board: &Board, point: &Point) -> bool {
    // SAFETY: The board is guaranteed to have dimensions of [BOARD_SIZE, BOARD_SIZE, BOARD_SIZE].
    // The Point components are usizes, so they are constrained to positive numbers.
    // Since we do a bounds check up front, there is no need for ndarray to perform its own bounds checking again.
    // This was shown to be a hot spot in profiling.
    point.x < BOARD_SIZE && point.y < BOARD_SIZE && point.z < BOARD_SIZE && unsafe { !board.occupied.uget([point.x, point.y, point.z]) }
}

pub fn get_points_for_orientation(point: &Point, orientation: &Orientation) -> [Point; 4] {
    let mut points: [Point; 4] = [*point; 4];

    // These orientations are all expressed as offsets from the center point of the piece.
    match orientation {
        Orientation::FlatUp => {
            points[1].x = point.x.wrapping_sub(1);
            points[2].x = point.x + 1;
            points[3].y = point.y.wrapping_sub(1);
        }
        Orientation::FlatLeft => {
            points[1].x = point.x.wrapping_sub(1);
            points[2].y = point.y.wrapping_sub(1);
            points[3].y = point.y + 1;
        }
        Orientation::FlatDown => {
            points[1].x = point.x.wrapping_sub(1);
            points[2].x = point.x + 1;
            points[3].y = point.y + 1;
        }
        Orientation::FlatRight => {
            points[1].y = point.y.wrapping_sub(1);
            points[2].y = point.y + 1;
            points[3].x = point.x + 1;
        }
        Orientation::FacedownHorizontal => {
            points[1].x = point.x.wrapping_sub(1);
            points[2].x = point.x + 1;
            points[3].z = point.z.wrapping_sub(1);
        }
        Orientation::FacedownVertical => {
            points[1].y = point.y.wrapping_sub(1);
            points[2].y = point.y + 1;
            points[3].z = point.z.wrapping_sub(1);
        }
        Orientation::FaceupHorizontal => {
            points[1].x = point.x.wrapping_sub(1);
            points[2].x = point.x + 1;
            points[3].z = point.z + 1;
        }
        Orientation::FaceupVertical => {
            points[1].y = point.y.wrapping_sub(1);
            points[2].y = point.y + 1;
            points[3].z = point.z + 1;
        }
        Orientation::UprightUp => {
            points[1].z = point.z + 1;
            points[2].z = point.z.wrapping_sub(1);
            points[3].y = point.y.wrapping_sub(1);
        }
        Orientation::UprightLeft => {
            points[1].z = point.z + 1;
            points[2].z = point.z.wrapping_sub(1);
            points[3].x = point.x.wrapping_sub(1);
        }
        Orientation::UprightDown => {
            points[1].z = point.z + 1;
            points[2].z = point.z.wrapping_sub(1);
            points[3].y = point.y + 1;
        }
        Orientation::UprightRight => {
            points[1].z = point.z + 1;
            points[2].z = point.z.wrapping_sub(1);
            points[3].x = point.x + 1;
        }
    }
    points
}
