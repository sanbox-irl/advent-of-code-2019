fn main() {
    let mut input: Vec<i32> = "1101,100,-1,4,0"
        .trim()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut instruction = 0;
    while let Some(move_amount) = process_instruction(instruction, &mut input) {
        instruction += move_amount;
        if instruction >= input.len() {
            break;
        }
    }

    println!("{:?}", input);
}

fn process_instruction(i_pos: usize, byte_array: &mut Vec<i32>) -> Option<usize> {
    let digitized_instruction = digitize_number(byte_array[i_pos] as u32);
    let instruction: Instruction = digitized_instruction[0].into();

    println!("{:?}", digitized_instruction);

    match instruction {
        Instruction::Add => {
            let val1 = parameter_value(digitized_instruction[1], byte_array[i_pos + 1], byte_array);
            let val2 = parameter_value(digitized_instruction[2], byte_array[i_pos + 2], byte_array);

            println!("gah");
            let final_position = byte_array[i_pos + 3] as usize;
            byte_array[final_position] = val1 + val2;

            Some(4)
        }

        Instruction::Multiply => unimplemented!(),

        Instruction::Halt => None,
    }
}

// 1 => {
//     true
// }
// 2 => {
//     let mult =
//         computer[computer[opcode_position + 1]] * computer[computer[opcode_position + 2]];
//     let final_position = computer[opcode_position + 3];
//     computer[final_position] = mult;
//     true
// }
// 99 => false,
// _ => panic!("Invalid opcode!"),

fn parameter_value(param_kind: u32, argument: i32, byte_array: &Vec<i32>) -> i32 {
    println!("Parameter Kind is {}", param_kind);
    println!("Argument is {}", argument);
    println!("Byte Array os {:?}", byte_array);
    match ParameterMode::from(param_kind) {
        ParameterMode::Pointer => byte_array[argument as usize],
        ParameterMode::Value => argument,
    }
}

const MAX_PARAMETER: usize = 4;

fn digitize_number(mut instruction: u32) -> (Instruction, [u32; MAX_PARAMETER]) {
    let mut array = [0; MAX_PARAMETER + 1];
    for digit in 0..MAX_PARAMETER + 1 {
        array[digit] = instruction % 10;
        instruction /= 10;
    }

    let mut final_array_idiot_double_digit_bullshit = [0; MAX_PARAMETER];
    final_array_idiot_double_digit_bullshit[0] = array[1] * 10 + array[2];
    final_array_idiot_double_digit_bullshit.copy_from_slice(&array[2..]);

    final_array_idiot_double_digit_bullshit
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}

impl From<u32> for Instruction {
    fn from(o: u32) -> Instruction {
        match o {
            1 => Instruction::Add,
            2 => Instruction::Multiply,
            99 => Instruction::Halt,
            _ => panic!("That is an invalid instruction: {}!", o),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum ParameterMode {
    Pointer = 0,
    Value = 1,
}

impl From<u32> for ParameterMode {
    fn from(o: u32) -> ParameterMode {
        match o {
            0 => ParameterMode::Pointer,
            1 => ParameterMode::Value,
            _ => panic!("That is an invalid parameter mode!"),
        }
    }
}
