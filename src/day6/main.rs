fn main() {
    let input: Vec<MassInstruction> = include_str!("input.txt")
        .trim()
        .lines()
        .map(|s| {
            let inputs: Vec<&str> = s.split(")").collect();
            MassInstruction {
                center: inputs[0],
                orbiter: inputs[1],
            }
        })
        .collect();

    let com = build_tree(input);
    let mut traversals = vec![];
    traverse_from_you_to_santa(&com, &mut traversals);

    traversals.sort();
    println!("Traversals needed {:?}", traversals);
}

fn traverse_from_you_to_santa(mass: &Mass, traversals: &mut Vec<usize>) {
    for child in mass.children.iter() {
        if let (Some(to_you), Some(to_santa)) = (
            find_distance_to_name(child, 0, "YOU"),
            find_distance_to_name(child, 0, "SAN"),
        ) {
            traversals.push(to_you + to_santa - 2);
            traverse_from_you_to_santa(child, traversals);
            break;
        }
    }
}

fn find_distance_to_name(
    mass: &Mass,
    mut current_count: usize,
    name_to_find: &str,
) -> Option<usize> {
    current_count += 1;

    match mass.children.len() {
        0 => None,
        _ => {
            for child in mass.children.iter() {
                if child.name == name_to_find {
                    return Some(current_count);
                } else {
                    if let Some(count) = find_distance_to_name(child, current_count, name_to_find) {
                        return Some(count);
                    }
                }
            }

            None
        }
    }
}

// fn count_it_up(mass: &Mass, mut current_count: usize) -> usize {
//     current_count += 1;
//     let count = current_count;

//     for child in mass.children.iter() {
//         current_count += count_it_up(child, count);
//     }

//     current_count
// }

fn build_tree(mut mass_instructions: Vec<MassInstruction>) -> Mass {
    // Take everything out of our arrays and make it into a tree!
    let mut com = Mass {
        name: "COM",
        children: vec![],
    };

    while mass_instructions.len() > 0 {
        for i in (0..mass_instructions.len()).rev() {
            if add_child(&mut com, &mass_instructions[i]) {
                mass_instructions.remove(i);
            }
        }
    }

    com
}

fn add_child(mass: &mut Mass, new_child: &MassInstruction) -> bool {
    if mass.name == new_child.center {
        mass.children.push(Mass {
            name: new_child.orbiter,
            children: vec![],
        });
        true
    } else {
        for child in mass.children.iter_mut() {
            if add_child(child, new_child) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MassInstruction {
    pub center: &'static str,
    pub orbiter: &'static str,
}

#[derive(Debug, Clone)]
pub struct Mass {
    pub name: &'static str,
    pub children: Vec<Mass>,
}
