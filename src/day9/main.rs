fn main() {
    let input: Vec<i64> = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut intcode = IntCode::new(input);
    intcode.execute(&mut || 2, &mut |out| println!("{}", out));
    // println!("{:?}", intcode.instructions);
}

pub struct IntCode {
    pub instructions: Vec<i64>,
    pub instruction_pointer: usize,
    pub relative_instruction_pointer: i64,
}

impl IntCode {
    pub fn new(instructions: Vec<i64>) -> IntCode {
        IntCode {
            instructions,
            instruction_pointer: 0,
            relative_instruction_pointer: 0,
        }
    }

    fn execute(&mut self, on_input: &mut dyn FnMut() -> i64, on_output: &mut dyn FnMut(i64)) {
        self.instruction_pointer = 0;
        self.relative_instruction_pointer = 0;

        while let Some(move_amount) = self.process_instruction(on_input, on_output) {
            self.instruction_pointer = move_amount;
        }
    }

    fn process_instruction(
        &mut self,
        on_input: &mut dyn FnMut() -> i64,
        on_output: &mut dyn FnMut(i64),
    ) -> Option<usize> {
        if self.read(self.instruction_pointer) == 21107 {
            let z = 10;
        }
        let (instruction, parameter_types) =
            Self::create_instruction_and_parameter(self.read(self.instruction_pointer) as u32);

        match instruction {
            Instruction::Add => {
                let val1 = self.parameter_value(parameter_types[0], 1);
                let val2 = self.parameter_value(parameter_types[1], 2);

                let final_position = self.write_to_parameter(parameter_types[2], 3);

                self.write(final_position, val1 + val2);

                Some(self.instruction_pointer + 4)
            }

            Instruction::Multiply => {
                let val1 = self.parameter_value(parameter_types[0], 1);
                let val2 = self.parameter_value(parameter_types[1], 2);

                let final_position = self.write_to_parameter(parameter_types[2], 3);

                self.write(final_position, val1 * val2);

                Some(self.instruction_pointer + 4)
            }

            Instruction::Input => {
                let pos = self.write_to_parameter(parameter_types[0], 1);
                self.write(pos, on_input());
                Some(self.instruction_pointer + 2)
            }

            Instruction::Output => {
                let value = self.parameter_value(parameter_types[0], 1);
                on_output(value);
                Some(self.instruction_pointer + 2)
            }

            Instruction::JumpTrue => {
                let test_param = self.parameter_value(parameter_types[0], 1);
                if test_param != 0 {
                    Some(self.parameter_value(parameter_types[1], 2) as usize)
                } else {
                    Some(self.instruction_pointer + 3)
                }
            }

            Instruction::JumpFalse => {
                let test_param = self.parameter_value(parameter_types[0], 1);
                if test_param == 0 {
                    Some(self.parameter_value(parameter_types[1], 2) as usize)
                } else {
                    Some(self.instruction_pointer + 3)
                }
            }

            Instruction::LessThan => {
                let param1 = self.parameter_value(parameter_types[0], 1);
                let param2 = self.parameter_value(parameter_types[1], 2);
                let val = if param1 < param2 { 1 } else { 0 };

                let pos = self.write_to_parameter(parameter_types[2], 3);
                self.write(pos, val);

                Some(self.instruction_pointer + 4)
            }

            Instruction::Equals => {
                let param1 = self.parameter_value(parameter_types[0], 1);
                let param2 = self.parameter_value(parameter_types[1], 2);
                let val = if param1 == param2 { 1 } else { 0 };

                let pos = self.write_to_parameter(parameter_types[2], 3);

                self.write(pos, val);

                Some(self.instruction_pointer + 4)
            }

            Instruction::RelativeBaseOffset => {
                let param1 = self.parameter_value(parameter_types[0], 1);
                self.relative_instruction_pointer += param1;

                Some(self.instruction_pointer + 2)
            }

            Instruction::Halt => None,
        }
    }

    fn parameter_value(&mut self, param_kind: ParameterMode, offset: usize) -> i64 {
        let argument = self.read(self.instruction_pointer + offset);
        match param_kind {
            ParameterMode::Position => self.read(argument as usize),
            ParameterMode::Immediate => argument,
            ParameterMode::Relative => {
                self.read((argument + self.relative_instruction_pointer) as usize)
            }
        }
    }

    fn write_to_parameter(&mut self, param_kind: ParameterMode, offset: usize) -> usize {
        let out = self.read(self.instruction_pointer + offset);
        let final_out = match param_kind {
            ParameterMode::Position => out,
            ParameterMode::Immediate => {
                unimplemented!("We should never have a self.Write-To parameter in immediate mode!")
            }
            ParameterMode::Relative => out + self.relative_instruction_pointer,
        };

        final_out as usize
    }

    fn read(&mut self, index: usize) -> i64 {
        if index >= self.instructions.len() {
            // extend our memory
            for _ in self.instructions.len() - 1..index * 2 {
                self.instructions.push(0);
            }
        }
        self.instructions[index]
    }

    fn write(&mut self, index: usize, value: i64) {
        if index >= self.instructions.len() {
            // extend our memory
            for _ in self.instructions.len() - 1..index {
                self.instructions.push(0);
            }
        }
        self.instructions[index] = value;
    }

    const MAX_PARAMETER: usize = 3;

    fn create_instruction_and_parameter(
        mut instruction: u32,
    ) -> (Instruction, [ParameterMode; Self::MAX_PARAMETER]) {
        let mut array = [0; Self::MAX_PARAMETER + 2];
        for digit in 0..Self::MAX_PARAMETER + 2 {
            array[digit] = instruction % 10;
            instruction /= 10;
        }

        let mut final_array_idiot_double_digit_bullshit =
            [ParameterMode::Position; Self::MAX_PARAMETER];

        for i in 2..Self::MAX_PARAMETER + 2 {
            final_array_idiot_double_digit_bullshit[i - 2] = array[i].into();
        }

        (
            (array[1] * 10 + array[0]).into(),
            final_array_idiot_double_digit_bullshit,
        )
    }
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
            5 => Instruction::JumpTrue,
            6 => Instruction::JumpFalse,
            7 => Instruction::LessThan,
            8 => Instruction::Equals,
            9 => Instruction::RelativeBaseOffset,
            99 => Instruction::Halt,

            _ => panic!("That is an invalid instruction: {}!", o),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl From<u32> for ParameterMode {
    fn from(o: u32) -> ParameterMode {
        match o {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("That is an invalid parameter mode!"),
        }
    }
}
