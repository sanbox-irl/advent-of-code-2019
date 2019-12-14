fn main() {
    let mut input: Vec<i64> = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    intcode(&mut input, &mut || 1, &mut |out| {
        println!("Output is: {}", out)
    });

    // println!("{:?}", input);
}

#[allow(dead_code)]
fn intcode(
    inputs: &mut Vec<i64>,
    on_input: &mut dyn FnMut() -> i64,
    on_output: &mut dyn FnMut(i64),
) {
    let mut instruction = 0;
    let mut relative_position = 0;
    while let Some(move_amount) = process_instruction(
        instruction,
        &mut relative_position,
        inputs,
        on_input,
        on_output,
    ) {
        instruction = move_amount;
        if instruction >= inputs.len() {
            break;
        }
    }
}

fn process_instruction(
    i_pos: usize,
    r_pos: &mut i64,
    byte_array: &mut Vec<i64>,
    on_input: &mut dyn FnMut() -> i64,
    on_output: &mut dyn FnMut(i64),
) -> Option<usize> {
    let (instruction, parameter_types) = create_instruction_and_parameter(byte_array[i_pos] as u32);

    match instruction {
        Instruction::Add => {
            let val1 =
                parameter_value(parameter_types[0], r_pos, byte_array[i_pos + 1], byte_array);
            let val2 =
                parameter_value(parameter_types[1], r_pos, byte_array[i_pos + 2], byte_array);

            let final_position = if parameter_types[2] != ParameterMode::Pointer {
                parameter_value(parameter_types[2], r_pos, byte_array[i_pos + 3], byte_array);

            } else {
                let final_pos = byte_array[i_pos + 3];
                byte_array[final_pos];
            }
            byte_array[final_position as usize] = val1 + val2;

            Some(i_pos + 4)
        }

        Instruction::Multiply => {
            let val1 =
                parameter_value(parameter_types[0], r_pos, byte_array[i_pos + 1], byte_array);
            let val2 =
                parameter_value(parameter_types[1], r_pos, byte_array[i_pos + 2], byte_array);

            let final_position =
                parameter_value(parameter_types[2], r_pos, byte_array[i_pos + 3], byte_array);
            byte_array[final_position as usize] = val1 * val2;

            Some(i_pos + 4)
        }

        Instruction::Input => {
            let pos = byte_array[i_pos + 1] as usize;
            byte_array[pos] = on_input();
            Some(i_pos + 2)
        }

        Instruction::Output => {
            let value =
                parameter_value(parameter_types[0], r_pos, byte_array[i_pos + 1], byte_array);
            on_output(value);
            Some(i_pos + 2)
        }

        Instruction::JumpTrue => {
            let test_param =
                parameter_value(parameter_types[0], r_pos, byte_array[i_pos + 1], byte_array);
            if test_param != 0 {
                Some(
                    parameter_value(parameter_types[1], r_pos, byte_array[i_pos + 2], byte_array)
                        as usize,
                )
            } else {
                Some(i_pos + 3)
            }
        }

        Instruction::JumpFalse => {
            let test_param =
                parameter_value(parameter_types[0], r_pos, byte_array[i_pos + 1], byte_array);
            if test_param == 0 {
                Some(
                    parameter_value(parameter_types[1], r_pos, byte_array[i_pos + 2], byte_array)
                        as usize,
                )
            } else {
                Some(i_pos + 3)
            }
        }

        Instruction::LessThan => {
            let param1 =
                parameter_value(parameter_types[0], r_pos, byte_array[i_pos + 1], byte_array);
            let param2 =
                parameter_value(parameter_types[1], r_pos, byte_array[i_pos + 2], byte_array);
            let val = if param1 < param2 { 1 } else { 0 };
            let pos = byte_array[i_pos + 3] as usize;
            byte_array[pos] = val;

            Some(i_pos + 4)
        }

        Instruction::Equals => {
            let param1 =
                parameter_value(parameter_types[0], r_pos, byte_array[i_pos + 1], byte_array);
            let param2 =
                parameter_value(parameter_types[1], r_pos, byte_array[i_pos + 2], byte_array);
            let val = if param1 == param2 { 1 } else { 0 };
            let pos = byte_array[i_pos + 3] as usize;
            byte_array[pos] = val;

            Some(i_pos + 4)
        }

        Instruction::RelativeBaseOffset => {
            let param1 =
                parameter_value(parameter_types[0], r_pos, byte_array[i_pos + 1], byte_array);

            *r_pos = param1;

            Some(i_pos + 2)
        }

        Instruction::Halt => None,
    }
}

fn parameter_value(
    param_kind: ParameterMode,
    r_pos: &mut i64,
    argument: i64,
    byte_array: &mut Vec<i64>,
) -> i64 {
    match param_kind {
        ParameterMode::Pointer => {
            let argument_u = argument as usize;
            if argument_u < byte_array.len() {
                byte_array[argument_u]
            } else {
                // extend our memory
                for _ in byte_array.len() - 1..argument_u * 2 {
                    byte_array.push(0);
                }

                byte_array[argument_u]
            }
        }
        ParameterMode::Value => argument,
        ParameterMode::Relative => argument + *r_pos,
    }
}

const MAX_PARAMETER: usize = 3;

fn create_instruction_and_parameter(
    mut instruction: u32,
) -> (Instruction, [ParameterMode; MAX_PARAMETER]) {
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

// fn digitize_number(mut number: i64) -> [i64; AMP_NUMBER] {
//     let mut array = [0; AMP_NUMBER];
//     for digit in 0..AMP_NUMBER {
//         array[digit] = number % 10;
//         number /= 10;
//     }

//     array
// }

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
    RelativeBaseOffset = 9,
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
            9 => Instruction::RelativeBaseOffset,

            _ => panic!("That is an invalid instruction: {}!", o),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum ParameterMode {
    Pointer = 0,
    Value = 1,
    Relative = 2,
}

impl From<u32> for ParameterMode {
    fn from(o: u32) -> ParameterMode {
        match o {
            0 => ParameterMode::Pointer,
            1 => ParameterMode::Value,
            2 => ParameterMode::Relative,
            _ => panic!("That is an invalid parameter mode!"),
        }
    }
}
