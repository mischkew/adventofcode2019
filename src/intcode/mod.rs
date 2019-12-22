use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
pub struct Intcode {
    data: Vec<i32>,
    position: usize,
}

#[derive(Debug, PartialEq)]
enum Parameter {
    Position(i32),
    Immediate(i32),
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl Intcode {
    pub fn new(data: Vec<i32>) -> Intcode {
        Intcode { data, position: 0 }
    }

    pub fn from_file(filename: &str) -> Intcode {
        let mut io = File::open(filename).expect("File not opened.");
        let mut contents = String::new();

        io.read_to_string(&mut contents)
            .expect("Failed to read file.");

        let data = contents
            .trim_end()
            .split(",")
            .map(|a| a.parse().unwrap())
            .collect();
        Intcode { data, position: 0 }
    }

    fn read_at(&self, position: u32) -> i32 {
        self.data[position as usize]
    }

    fn write_at(&mut self, position: u32, value: i32) {
        self.data[position as usize] = value;
    }

    fn read(&mut self, count: u32) -> &[i32] {
        let ints = &self.data[self.position..self.position + (count as usize)];
        self.position = self.position + count as usize;

        ints
    }

    fn jump(&mut self, position: usize) {
        self.position = position;
    }

    fn get_input(&self, parameter: &Parameter) -> i32 {
        match parameter {
            Parameter::Immediate(value) => *value,
            Parameter::Position(value) => self.read_at(*value as u32),
        }
    }

    fn get_target(&self, parameter: &Parameter) -> u32 {
        match parameter {
            Parameter::Immediate(_) => panic!("Target parameter should be in position mode!"),
            Parameter::Position(value) => *value as u32,
        }
    }

    pub fn read_one(&mut self) -> i32 {
        self.read(1)[0]
    }

    pub fn run(&mut self, inputs: Vec<i32>) -> Vec<i32> {
        let mut inputs = inputs.iter();
        let mut outputs = vec![];

        loop {
            let (operation, parameters) = self.next();

            match operation {
                Operation::Add => {
                    assert_eq!(parameters.len(), 3);

                    let a = self.get_input(&parameters[0]);
                    let b = self.get_input(&parameters[1]);
                    self.write_at(self.get_target(&parameters[2]), a + b);
                }
                Operation::Multiply => {
                    assert_eq!(parameters.len(), 3);

                    let a = self.get_input(&parameters[0]);
                    let b = self.get_input(&parameters[1]);
                    self.write_at(self.get_target(&parameters[2]), a * b);
                }
                Operation::Input => {
                    assert_eq!(parameters.len(), 1);
                    if let Some(input) = inputs.next() {
                        self.write_at(self.get_target(&parameters[0]), *input);
                    } else {
                        panic!("Too few inputs provided!");
                    }
                }
                Operation::Output => {
                    assert_eq!(parameters.len(), 1);
                    outputs.push(self.get_input(&parameters[0]));
                }
                Operation::JumpIfTrue => {
                    assert_eq!(parameters.len(), 2);

                    let condition = self.get_input(&parameters[0]);
                    let jump_position = self.get_input(&parameters[1]);

                    if condition != 0 {
                        self.jump(jump_position as usize);
                    }
                }
                Operation::JumpIfFalse => {
                    assert_eq!(parameters.len(), 2);

                    let condition = self.get_input(&parameters[0]);
                    let jump_position = self.get_input(&parameters[1]);

                    if condition == 0 {
                        self.jump(jump_position as usize);
                    }
                }
                Operation::LessThan => {
                    assert_eq!(parameters.len(), 3);

                    let a = self.get_input(&parameters[0]);
                    let b = self.get_input(&parameters[1]);
                    let result = if a < b { 1 } else { 0 };
                    self.write_at(self.get_target(&parameters[2]), result);
                }
                Operation::Equals => {
                    assert_eq!(parameters.len(), 3);

                    let a = self.get_input(&parameters[0]);
                    let b = self.get_input(&parameters[1]);
                    let result = if a == b { 1 } else { 0 };
                    self.write_at(self.get_target(&parameters[2]), result);
                }
                Operation::Halt => break,
            }
        }

        outputs
    }

    fn next(&mut self) -> (Operation, Vec<Parameter>) {
        let parameter_code = self.read_one();
        let mut remainder = parameter_code / 100;
        let opcode = parameter_code % 100;
        let mut parameters: Vec<Parameter> = Vec::new();

        let (operation, size) = match opcode {
            1 => (Operation::Add, 3),
            2 => (Operation::Multiply, 3),
            3 => (Operation::Input, 1),
            4 => (Operation::Output, 1),
            5 => (Operation::JumpIfTrue, 2),
            6 => (Operation::JumpIfFalse, 2),
            7 => (Operation::LessThan, 3),
            8 => (Operation::Equals, 3),
            99 => (Operation::Halt, 0),
            _ => panic!("Unknown operation {}", opcode),
        };

        for _ in 0..size {
            let mode = match remainder % 10 {
                0 => Parameter::Position,
                1 => Parameter::Immediate,
                _ => panic!("Unknown mode {}", remainder % 10),
            };
            remainder = remainder / 10;

            let parameter = mode(self.read_one());
            parameters.push(parameter);
        }

        (operation, parameters)
    }
}
