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
    Halt,
}

impl Intcode {
    pub fn new(data: Vec<i32>) -> Intcode {
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

    pub fn run(&mut self) {
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
                    self.write_at(self.get_target(&parameters[0]), 1);
                }
                Operation::Output => {
                    assert_eq!(parameters.len(), 1);
                    println!("Output: {}", self.get_input(&parameters[0]));
                }
                Operation::Halt => break,
            }
        }
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
