const INPUT: i32 = 5;

fn main() {
    let mut input: Vec<i32> = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut instruction = 0;
    while let Some(move_amount) = process_instruction(instruction, &mut input) {
        instruction = move_amount;
        if instruction >= input.len() {
            break;
        }
    }
}

fn process_instruction(i_pos: usize, byte_array: &mut Vec<i32>) -> Option<usize> {
    let (instruction, parameter_types) = digitize_number(byte_array[i_pos] as u32);

    match instruction {
        Instruction::Add => {
            let val1 = parameter_value(parameter_types[0], byte_array[i_pos + 1], byte_array);
            let val2 = parameter_value(parameter_types[1], byte_array[i_pos + 2], byte_array);

            let final_position = byte_array[i_pos + 3] as usize;
            byte_array[final_position] = val1 + val2;

            Some(i_pos + 4)
        }

        Instruction::Multiply => {
            let val1 = parameter_value(parameter_types[0], byte_array[i_pos + 1], byte_array);
            let val2 = parameter_value(parameter_types[1], byte_array[i_pos + 2], byte_array);

            let final_position = byte_array[i_pos + 3] as usize;
            byte_array[final_position] = val1 * val2;

            Some(i_pos + 4)
        }

        Instruction::Input => {
            println!("Input is {}", INPUT);
            let pos = byte_array[i_pos + 1] as usize;
            byte_array[pos] = INPUT;
            Some(i_pos + 2)
        }

        Instruction::Output => {
            let value = parameter_value(parameter_types[0], byte_array[i_pos + 1], byte_array);
            println!("OUTPUT is: {}", value);
            Some(i_pos + 2)
        }

        Instruction::JumpTrue => {
            let test_param = parameter_value(parameter_types[0], byte_array[i_pos + 1], byte_array);
            if test_param != 0 {
                Some(
                    parameter_value(parameter_types[1], byte_array[i_pos + 2], byte_array) as usize,
                )
            } else {
                Some(i_pos + 3)
            }
        }

        Instruction::JumpFalse => {
            let test_param = parameter_value(parameter_types[0], byte_array[i_pos + 1], byte_array);
            if test_param == 0 {
                Some(
                    parameter_value(parameter_types[1], byte_array[i_pos + 2], byte_array) as usize,
                )
            } else {
                Some(i_pos + 3)
            }
        }

        Instruction::LessThan => {
            let param1 = parameter_value(parameter_types[0], byte_array[i_pos + 1], byte_array);
            let param2 = parameter_value(parameter_types[1], byte_array[i_pos + 2], byte_array);
            let val = if param1 < param2 { 1 } else { 0 };
            let pos = byte_array[i_pos + 3] as usize;
            byte_array[pos] = val;

            Some(i_pos + 4)
        }

        Instruction::Equals => {
            let param1 = parameter_value(parameter_types[0], byte_array[i_pos + 1], byte_array);
            let param2 = parameter_value(parameter_types[1], byte_array[i_pos + 2], byte_array);
            let val = if param1 == param2 { 1 } else { 0 };
            let pos = byte_array[i_pos + 3] as usize;
            byte_array[pos] = val;

            Some(i_pos + 4)
        }

        Instruction::Halt => None,
    }
}

fn parameter_value(param_kind: ParameterMode, argument: i32, byte_array: &Vec<i32>) -> i32 {
    match param_kind {
        ParameterMode::Pointer => byte_array[argument as usize],
        ParameterMode::Value => argument,
    }
}

const MAX_PARAMETER: usize = 3;

fn digitize_number(mut instruction: u32) -> (Instruction, [ParameterMode; MAX_PARAMETER]) {
    let mut array = [0; MAX_PARAMETER + 2];
    for digit in 0..MAX_PARAMETER + 1 {
        array[digit] = instruction % 10;
        instruction /= 10;
    }

    let mut final_array_idiot_double_digit_bullshit = [ParameterMode::Pointer; MAX_PARAMETER];

    for i in 2..MAX_PARAMETER + 2 {
        final_array_idiot_double_digit_bullshit[i - 2] = array[i].into();
    }

    (
        (array[1] * 10 + array[0]).into(),
        final_array_idiot_double_digit_bullshit,
    )
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpTrue = 5,
    JumpFalse = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99,
}

impl From<u32> for Instruction {
    fn from(o: u32) -> Instruction {
        match o {
            1 => Instruction::Add,
            2 => Instruction::Multiply,
            3 => Instruction::Input,
            4 => Instruction::Output,
            99 => Instruction::Halt,
            5 => Instruction::JumpTrue,
            6 => Instruction::JumpFalse,
            7 => Instruction::LessThan,
            8 => Instruction::Equals,

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
