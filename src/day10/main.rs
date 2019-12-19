const FIELD_WIDTH: usize = 10;

fn main() {
    let mut input: Vec<Option<Asteroid>> = include_str!("input.txt")
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line_str)| {
            line_str.chars().enumerate().map(move |(x, c)| {
                if c == '#' {
                    Some(Asteroid::new((x, y)))
                } else {
                    None
                }
            })
        })
        .collect();
    let mut asteroid_view = input.clone();

    let position = part1(&mut input, &asteroid_view);
    let destruction_list: Vec<AsteroidRelationship> = input[position.0 + position.1 * FIELD_WIDTH]
        .as_ref()
        .unwrap()
        .asteroid_relationships
        .clone();

    for relationship in destruction_list.into_iter() {
        let position = relationship.other_position;
        input.remove(position.0 + position.1 * FIELD_WIDTH);
        asteroid_view
    }

}

fn part1(input: &mut Vec<Option<Asteroid>>, asteroid_view: &Vec<Option<Asteroid>>) -> Position {
    asteroid_procedure(input, asteroid_view);

    let mut final_pos = (0, 0);
    let mut best_score = 0;

    for asteroid in input.iter() {
        if let Some(asteroid) = asteroid {
            if asteroid.asteroid_relationships.len() > best_score {
                best_score = asteroid.asteroid_relationships.len();
                final_pos = asteroid.position;
            }
        }
    }

    final_pos
}

fn asteroid_procedure(input: &mut Vec<Option<Asteroid>>, asteroid_view: &Vec<Option<Asteroid>>) {
    // Make Basic Views
    for asteroid in input.iter_mut() {
        if let Some(asteroid) = asteroid {
            for that_asteroid in asteroid_view.iter() {
                if let Some(that_asteroid) = that_asteroid {
                    if that_asteroid.position != asteroid.position {
                        make_relationship(asteroid, that_asteroid);
                    }
                }
            }
        }
    }

    // Deduplicate the Views:
    for asteroid in input.iter_mut() {
        if let Some(asteroid) = asteroid {
            let asteroid: &mut Asteroid = asteroid;
            asteroid.asteroid_relationships.sort_by(|la, ra| {
                la.angle
                    .partial_cmp(&ra.angle)
                    .and_then(|ord| {
                        if ord == std::cmp::Ordering::Equal {
                            Some(ra.distance.partial_cmp(&la.distance).unwrap())
                        } else {
                            Some(ord)
                        }
                    })
                    .unwrap()
            });
            asteroid
                .asteroid_relationships
                .dedup_by_key(|asteroid| asteroid.angle);
        }
    }
}

fn make_relationship(asteroid: &mut Asteroid, other: &Asteroid) {
    let distance_vec2 = (
        other.position.0 as f64 - asteroid.position.0 as f64,
        asteroid.position.1 as f64 - other.position.1 as f64,
    );
    let distance = (distance_vec2.0 * distance_vec2.0 + distance_vec2.1 * distance_vec2.1).sqrt();

    let angle = distance_vec2.1.atan2(distance_vec2.0);

    asteroid.asteroid_relationships.push(AsteroidRelationship {
        distance,
        angle,
        other_position: other.position,
    })
}

#[derive(Debug, Clone)]
struct Asteroid {
    pub position: Position,
    pub asteroid_relationships: Vec<AsteroidRelationship>,
}

impl Asteroid {
    pub fn new(position: Position) -> Asteroid {
        Asteroid {
            position,
            asteroid_relationships: vec![],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct AsteroidRelationship {
    pub other_position: Position,
    pub distance: f64,
    pub angle: f64,
}

pub type Position = (usize, usize);
