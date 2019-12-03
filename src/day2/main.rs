pub fn main() {
    let input: Vec<usize> = include_str!("input.txt")
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    // part1
    run_intcode(input.clone(), Parameters { noun: 12, verb: 2 });

    // part2
    part2(input);
}

pub fn process_instruction(opcode_position: usize, computer: &mut Vec<usize>) -> bool {
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

pub fn part2(instructions: Vec<usize>) {
    const MAGIC_GOAL: usize = 19690720;
    let mut final_instructions = None;

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let params = Parameters { noun, verb };

            if run_intcode(instructions.clone(), params) == MAGIC_GOAL {
                final_instructions = Some(params);
                break 'outer;
            }
        }
    }

    println!("Succesful codes are {:?}", final_instructions);
}

pub fn run_intcode(mut instructions: Vec<usize>, params: Parameters) -> usize {
    // Instructions provided by AoC
    instructions[1] = params.noun;
    instructions[2] = params.verb;

    let mut opcode_position = 0;
    loop {
        if process_instruction(opcode_position, &mut instructions) == false {
            break;
        }
        opcode_position += 4;
    }

    instructions[0]
}

#[derive(Debug, Clone, Copy)]
pub struct Parameters {
    pub noun: usize,
    pub verb: usize,
}
