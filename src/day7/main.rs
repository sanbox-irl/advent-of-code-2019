use std::sync::mpsc;
use std::thread;

const AMP_NUMBER: usize = 5;

fn main() {
    let start = std::time::Instant::now();

    let input: Vec<i32> = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let output = heap_algorithm(&mut [5, 6, 7, 8, 9])
        .iter()
        .map(|signal| run_simulation_looped(&input, signal))
        .max()
        .unwrap_or_default();

    println!("Max Thruster Signal is {}", output);
    let diff: std::time::Duration = std::time::Instant::now() - start;
    println!("Miliseconds: {}", diff.as_secs_f64()*1000.0)
}

fn heap_algorithm(list: &mut [i32; AMP_NUMBER]) -> Vec<[i32; AMP_NUMBER]> {
    let mut ret = vec![];

    generate(AMP_NUMBER, list, &mut ret);

    fn generate(n: usize, array: &mut [i32; AMP_NUMBER], heap: &mut Vec<[i32; AMP_NUMBER]>) {
        if n == 1 {
            heap.push(array.clone());
        } else {
            for i in 0..n - 1 {
                generate(n - 1, array, heap);

                if n % 2 == 0 {
                    array.swap(i, n - 1);
                } else {
                    array.swap(0, n - 1);
                }
            }
            generate(n - 1, array, heap);
        }
    }

    ret
}

#[allow(dead_code)]
fn run_simulation(input: &Vec<i32>, initial_signals: [i32; AMP_NUMBER]) -> i32 {
    let mut senders = vec![];
    let mut receivers = vec![];
    for i in 0..AMP_NUMBER {
        let (sender, receiver) = mpsc::channel();

        sender.send(initial_signals[i]).unwrap();
        if i == 0 {
            sender.send(0).unwrap();
        }

        senders.push(Some(sender));
        receivers.push(Some(receiver));
    }

    // Final Version...
    let (final_in, final_out) = mpsc::channel();
    senders.push(Some(final_in));

    // Create Amplifiers!
    for i in 0..AMP_NUMBER {
        let mut input = input.clone();
        let outbound_channel = receivers[i].take().unwrap();
        let inbound_channel = senders[i + 1].take().unwrap();

        thread::spawn(move || {
            amplifier(
                &mut input,
                &mut || outbound_channel.recv().unwrap(),
                &mut |i| inbound_channel.send(i).unwrap(),
            );
        });
    }

    final_out.recv().unwrap()
}

#[allow(dead_code)]
fn run_simulation_looped(input: &Vec<i32>, initial_signals: &[i32; AMP_NUMBER]) -> i32 {
    let mut senders = vec![];
    let mut receivers = vec![];

    let mut initial_sender = None;

    for i in 0..AMP_NUMBER {
        let (sender, receiver) = mpsc::channel();

        sender.send(initial_signals[i]).unwrap();
        if i == 0 {
            sender.send(0).unwrap();
            initial_sender = Some(sender);
            senders.push(None);
        } else {
            senders.push(Some(sender));
        }

        receivers.push(Some(receiver));
    }

    // Final Version...
    let (final_in, final_out) = mpsc::channel();
    senders.push(Some(final_in));

    // Create Amplifiers!
    for i in 0..AMP_NUMBER {
        let mut input = input.clone();
        let outbound_channel = receivers[i].take().unwrap();
        let inbound_channel = senders[i + 1].take().unwrap();

        thread::spawn(move || {
            amplifier(
                &mut input,
                &mut || outbound_channel.recv().unwrap(),
                &mut |i| inbound_channel.send(i).unwrap(),
            );
        });
    }

    for recv in final_out {
        if let Some(sender) = &mut initial_sender {
            match sender.send(recv) {
                Ok(()) => {}
                Err(_) => return recv,
            }
        }
    }

    0
}

fn amplifier(
    inputs: &mut Vec<i32>,
    on_input: &mut dyn FnMut() -> i32,
    on_output: &mut dyn FnMut(i32),
) {
    let mut instruction = 0;
    while let Some(move_amount) = process_instruction(instruction, inputs, on_input, on_output) {
        instruction = move_amount;
        if instruction >= inputs.len() {
            break;
        }
    }
}

fn process_instruction(
    i_pos: usize,
    byte_array: &mut Vec<i32>,
    on_input: &mut dyn FnMut() -> i32,
    on_output: &mut dyn FnMut(i32),
) -> Option<usize> {
    let (instruction, parameter_types) = create_instruction_and_parameter(byte_array[i_pos] as u32);

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
            let pos = byte_array[i_pos + 1] as usize;
            byte_array[pos] = on_input();
            Some(i_pos + 2)
        }

        Instruction::Output => {
            let value = parameter_value(parameter_types[0], byte_array[i_pos + 1], byte_array);
            on_output(value);
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

// fn digitize_number(mut number: i32) -> [i32; AMP_NUMBER] {
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
