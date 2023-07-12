use crate::defs::*;

// Checks to see if a given point and orientation overhangs an empty point. If so, we don't want to place a piece there.
pub fn has_empty_overhang(board: &Board, point: &Point, orientation: &Orientation) -> bool {
    match orientation {
        Orientation::FacedownHorizontal => {
            inbounds_and_clear(
                &board,
                &Point {
                    x: point.x.wrapping_sub(1),
                    y: point.y,
                    z: point.z.wrapping_sub(1),
                },
            ) || inbounds_and_clear(
                &board,
                &Point {
                    x: point.x + 1,
                    y: point.y,
                    z: point.z.wrapping_sub(1),
                },
            )
        }
        Orientation::FacedownVertical => {
            inbounds_and_clear(
                &board,
                &Point {
                    x: point.x,
                    y: point.y.wrapping_sub(1),
                    z: point.z.wrapping_sub(1),
                },
            ) || inbounds_and_clear(
                &board,
                &Point {
                    x: point.x,
                    y: point.y + 1,
                    z: point.z.wrapping_sub(1),
                },
            )
        }
        Orientation::UprightUp => inbounds_and_clear(
            &board,
            &Point {
                x: point.x,
                y: point.y.wrapping_sub(1),
                z: point.z.wrapping_sub(1),
            },
        ),
        Orientation::UprightLeft => inbounds_and_clear(
            &board,
            &Point {
                x: point.x.wrapping_sub(1),
                y: point.y,
                z: point.z.wrapping_sub(1),
            },
        ),
        Orientation::UprightDown => inbounds_and_clear(
            &board,
            &Point {
                x: point.x,
                y: point.y + 1,
                z: point.z.wrapping_sub(1),
            },
        ),
        Orientation::UprightRight => inbounds_and_clear(
            &board,
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
    match orientation {
        Orientation::FacedownHorizontal | Orientation::FacedownVertical => true,
        Orientation::UprightUp | Orientation::UprightLeft | Orientation::UprightDown | Orientation::UprightRight => true,
        _ => false,
    }
}

pub fn inbounds_and_clear(board: &Board, point: &Point) -> bool {
    return point.x <= BOARD_SIZE - 1
        && point.y <= BOARD_SIZE - 1
        && point.z <= BOARD_SIZE - 1
        && !board.occupied[[point.x, point.y, point.z]];
}

pub fn inbounds_and_occupied(board: &Board, point: &Point) -> bool {
    return point.x <= BOARD_SIZE - 1
        && point.y <= BOARD_SIZE - 1
        && point.z <= BOARD_SIZE - 1
        && board.occupied[[point.x, point.y, point.z]];
}

pub fn get_points_for_orientation(point: &Point, orientation: &Orientation) -> [Point; 4] {
    let mut points: [Point; 4] = Default::default();
    points[0] = *point;

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
                z: point.z.wrapping_sub(1),
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
                z: point.z.wrapping_sub(1),
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
