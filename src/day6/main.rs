fn main() {
    let mut input: Vec<MassInstruction> = include_str!("input.txt")
        .lines()
        .map(|s| {
            let inputs: Vec<&str> = s.split(")").collect();
            MassInstruction {
                center: inputs[0],
                orbiter: inputs[1],
            }
        })
        .collect();

    // Take everything out of our arrays and make it into a tree!
    let COM = Mass {
        name: "COM",
        children: vec![],
    };

    while input.len() > 0 {
        for i in 0..input.len() {
            if 
        }
    } 
}

pub struct MassInstruction {
    pub center: &'static str,
    pub orbiter: &'static str,
}

pub struct Mass {
    pub name: &'static str,
    pub children: Vec<Mass>,
}
