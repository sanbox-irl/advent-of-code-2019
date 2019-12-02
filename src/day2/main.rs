pub fn main() {
    let mut input: Vec<usize> = include_str!("input.txt")
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut opcode_position = 0;
    loop {
        if process_opcode(opcode_position, &mut input) == false {
            break;
        }
        opcode_position += 4;
    }

    println!("Program looks like this now:\n{:?}", input);
}

pub fn process_opcode(opcode_position: usize, computer: &mut Vec<usize>) -> bool {
    match computer[opcode_position] {
        1 => {
            let added =
                computer[computer[opcode_position + 1]] + computer[computer[opcode_position + 2]];
            let final_position = computer[opcode_position + 3];
            computer[final_position] = added;
            true
        }
        2 => {
            let mult =
                computer[computer[opcode_position + 1]] * computer[computer[opcode_position + 2]];
            let final_position = computer[opcode_position + 3];
            computer[final_position] = mult;
            true
        }
        99 => false,
        _ => panic!("Invalid opcode!"),
    }
}
