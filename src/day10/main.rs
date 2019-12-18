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
    let asteroid_view = input.clone();

    let position = part1(&mut input, &asteroid_view);
}

fn part1(
    input: &mut Vec<Option<Asteroid>>,
    asteroid_view: &Vec<Option<Asteroid>>,
) -> (usize, usize) {
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
            asteroid.other_asteroids.sort_by(|la, ra| {
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
                .other_asteroids
                .dedup_by_key(|asteroid| asteroid.angle);
        }
    }

    let mut final_pos = (0, 0);
    let mut best_score = 0;

    for asteroid in input.iter() {
        if let Some(asteroid) = asteroid {
            if asteroid.other_asteroids.len() > best_score {
                best_score = asteroid.other_asteroids.len();
                final_pos = asteroid.position;
            }
        }
    }

    final_pos
}

fn make_relationship(asteroid: &mut Asteroid, other: &Asteroid) {
    let distance_vec2 = (
        other.position.0 as f64 - asteroid.position.0 as f64,
        asteroid.position.1 as f64 - other.position.1 as f64,
    );
    let distance = (distance_vec2.0 * distance_vec2.0 + distance_vec2.1 * distance_vec2.1).sqrt();

    let angle = distance_vec2.1.atan2(distance_vec2.0);

    asteroid
        .other_asteroids
        .push(AsteroidRelationship { distance, angle })
}

#[derive(Debug, Clone)]
struct Asteroid {
    pub position: (usize, usize),
    pub other_asteroids: Vec<AsteroidRelationship>,
}

impl Asteroid {
    pub fn new(position: (usize, usize)) -> Asteroid {
        Asteroid {
            position,
            other_asteroids: vec![],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct AsteroidRelationship {
    pub distance: f64,
    pub angle: f64,
}
