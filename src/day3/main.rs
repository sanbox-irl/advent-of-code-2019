fn main() {
    let wires: Vec<Vec<Point>> = include_str!("input.txt")
        .lines()
        .map(|line_str| line_str.split(",").collect::<Vec<&str>>())
        .map(|line_vec| line_vec.iter().map(|i| parse_instruction(i)).collect())
        .collect();
    assert_eq!(wires.len(), 2);

    let mut final_collisions: Vec<usize> = vec![];

    let mut red_start = Point { x: 0, y: 0 };
    let mut red_walking_total: usize = 0;

    for this_move in wires[0].iter() {
        let red_segment = Line::new(red_start, *this_move);

        // Compare to Green here
        {
            let mut green_start = Point { x: 0, y: 0 };
            let mut green_walking_total: usize = 0;

            for that_move in wires[1].iter() {
                let green_segment = Line::new(green_start, *that_move);

                let collision_happened = {
                    match green_segment.move_kind {
                        MoveKind::Horizontal => {
                            if red_segment.move_kind == MoveKind::Horizontal {
                                None // No collisions on likewise moving lines!
                            } else {
                                collision(green_segment, red_segment)
                            }
                        }

                        MoveKind::Vertical => {
                            if red_segment.move_kind == MoveKind::Vertical {
                                None // No collisions on likewise moving lines!
                            } else {
                                collision(red_segment, green_segment)
                            }
                        }
                    }
                };

                if let Some(col_point) = collision_happened {
                    let walk_to_collision = {
                        let red_walk =
                            (col_point.x - red_start.x + col_point.y - red_start.y).abs() as usize;

                        let green_walk = (col_point.x - green_start.x + col_point.y - green_start.y)
                            .abs() as usize;

                        red_walk + green_walk
                    };

                    final_collisions
                        .push(red_walking_total + green_walking_total + walk_to_collision);
                }

                green_start = Point {
                    x: green_start.x + that_move.x,
                    y: green_start.y + that_move.y,
                };

                green_walking_total += match green_segment.move_kind {
                    MoveKind::Horizontal => that_move.x.abs() as usize,
                    MoveKind::Vertical => that_move.y.abs() as usize,
                };
            }
        }
        red_start = Point {
            x: red_start.x + this_move.x,
            y: red_start.y + this_move.y,
        };

        red_walking_total += match red_segment.move_kind {
            MoveKind::Horizontal => this_move.x.abs() as usize,
            MoveKind::Vertical => this_move.y.abs() as usize,
        };
    }

    final_collisions.sort();

    println!("Sorted Distances are {:#?}", final_collisions);
}

fn parse_instruction(instruction: &str) -> Point {
    let first_letter = &instruction[0..1];
    let n: i32 = instruction[1..].parse().unwrap();

    match first_letter {
        "L" => Point { x: -n, y: 0 },
        "R" => Point { x: n, y: 0 },
        "U" => Point { x: 0, y: n },
        "D" => Point { x: 0, y: -n },
        _ => panic!("NANI??"),
    }
}

fn collision(h: Line, v: Line) -> Option<Point> {
    if (h.start.x < v.start.x && v.start.x - h.start.x <= h.move_amount)
        && (h.start.y > v.start.y && h.start.y - v.start.y <= v.move_amount)
    {
        Some(Point {
            x: v.start.x,
            y: h.start.y,
        })
    } else {
        None
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Line {
    pub start: Point,
    pub move_amount: i32,
    pub move_kind: MoveKind,
}

impl Line {
    pub fn new(last_point: Point, move_amount: Point) -> Line {
        let move_kind: MoveKind = if move_amount.x == 0 {
            MoveKind::Vertical
        } else {
            MoveKind::Horizontal
        };

        let (start, move_amount): (Point, i32) = match move_kind {
            MoveKind::Horizontal => {
                let start = if move_amount.x > 0 {
                    last_point
                } else {
                    Point {
                        x: last_point.x + move_amount.x,
                        ..last_point
                    }
                };

                (start, move_amount.x.abs())
            }

            MoveKind::Vertical => {
                let start = if move_amount.y > 0 {
                    last_point
                } else {
                    Point {
                        y: last_point.y + move_amount.y,
                        ..last_point
                    }
                };

                (start, move_amount.y.abs())
            }
        };

        Line {
            start,
            move_kind,
            move_amount,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MoveKind {
    Horizontal,
    Vertical,
}
