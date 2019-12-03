#[derive(Debug, PartialEq)]
enum Opcode {
    Add,
    Multiply,
    Halt,
}

impl Opcode {
    pub fn new(int: &usize) -> Result<Opcode, &'static str> {
        match int {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            99 => Ok(Opcode::Halt),
            _ => Err("Unknown Opcode."),
        }
    }

    fn execute(&self, position: usize, data: &mut Vec<usize>) -> usize {
        let input = data[data[position + 1]];
        let other_input = data[data[position + 2]];
        let target = data[position + 3];

        let value = match self {
            Self::Add => input + other_input,
            Self::Multiply => input * other_input,
            _ => 0,
        };

        data[target] = value;
        return position + 4;
    }

    fn process(data: &mut Vec<usize>) -> Result<usize, &'static str> {
        let mut position = 0;
        while position < data.len() {
            let optcode = Opcode::new(&data[position])?;

            if let Self::Halt = optcode {
                return Ok(data[0]);
            }
            position = optcode.execute(position, data);
        }
        Err("Reached end of data without encountering Halt (99).")
    }
}

fn restore_gravity(data: &mut Vec<usize>) {
    data[1] = 12;
    data[2] = 2;
}

fn restore_computer() -> Result<(), &'static str> {
    let mut data = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 9, 1, 19, 1, 19, 5, 23, 1, 23, 6, 27, 2,
        9, 27, 31, 1, 5, 31, 35, 1, 35, 10, 39, 1, 39, 10, 43, 2, 43, 9, 47, 1, 6, 47, 51, 2, 51,
        6, 55, 1, 5, 55, 59, 2, 59, 10, 63, 1, 9, 63, 67, 1, 9, 67, 71, 2, 71, 6, 75, 1, 5, 75, 79,
        1, 5, 79, 83, 1, 9, 83, 87, 2, 87, 10, 91, 2, 10, 91, 95, 1, 95, 9, 99, 2, 99, 9, 103, 2,
        10, 103, 107, 2, 9, 107, 111, 1, 111, 5, 115, 1, 115, 2, 119, 1, 119, 6, 0, 99, 2, 0, 14,
        0,
    ];

    println!("Restoring gravity...");
    restore_gravity(&mut data);

    println!("Running program...");
    let pos0 = Opcode::process(&mut data)?;

    println!("Done. First position is {}", pos0);
    Ok(())
}

fn solve_gravity_assist() -> Result<(), &'static str> {
    let initial_memory = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 9, 1, 19, 1, 19, 5, 23, 1, 23, 6, 27, 2,
        9, 27, 31, 1, 5, 31, 35, 1, 35, 10, 39, 1, 39, 10, 43, 2, 43, 9, 47, 1, 6, 47, 51, 2, 51,
        6, 55, 1, 5, 55, 59, 2, 59, 10, 63, 1, 9, 63, 67, 1, 9, 67, 71, 2, 71, 6, 75, 1, 5, 75, 79,
        1, 5, 79, 83, 1, 9, 83, 87, 2, 87, 10, 91, 2, 10, 91, 95, 1, 95, 9, 99, 2, 99, 9, 103, 2,
        10, 103, 107, 2, 9, 107, 111, 1, 111, 5, 115, 1, 115, 2, 119, 1, 119, 6, 0, 99, 2, 0, 14,
        0,
    ];

    for noun in 0..100 {
        for verb in 0..100 {
            let mut data = initial_memory.clone();
            data[1] = noun;
            data[2] = verb;

            if Opcode::process(&mut data)? == 19690720 {
                println!("Found valid inputs {}", 100 * noun + verb);
                return Ok(());
            }
        }
    }

    Ok(())
}

fn main() {
    restore_computer().expect("Failed to restore computer.");
    solve_gravity_assist().expect("Failed to solve gravity assist.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_optcode() {
        assert_eq!(Opcode::new(&1).unwrap(), Opcode::Add);
        assert_eq!(Opcode::new(&2).unwrap(), Opcode::Multiply);
        assert_eq!(Opcode::new(&99).unwrap(), Opcode::Halt);
    }

    #[test]
    #[should_panic]
    fn unknown_optcode() {
        Opcode::new(&3).unwrap();
    }

    #[test]
    fn execute_add() {
        let mut data = vec![1, 0, 3, 2];
        let new_position = Opcode::Add.execute(0, &mut data);

        assert_eq!(data[2], 3);
        assert_eq!(new_position, 4);
    }

    #[test]
    fn execute_mult() {
        let mut data = vec![0, 3, 2, 2];
        let new_position = Opcode::Multiply.execute(0, &mut data);

        assert_eq!(data[2], 4);
        assert_eq!(new_position, 4);
    }

    #[test]
    fn process_1() -> Result<(), &'static str> {
        let mut data = vec![1, 0, 0, 0, 99];
        Opcode::process(&mut data)?;

        assert_eq!(vec![2, 0, 0, 0, 99], data);
        Ok(())
    }

    #[test]
    fn process_2() -> Result<(), &'static str> {
        let mut data = vec![2, 3, 0, 3, 99];
        Opcode::process(&mut data)?;

        assert_eq!(vec![2, 3, 0, 6, 99], data);
        Ok(())
    }

    #[test]
    fn process_3() -> Result<(), &'static str> {
        let mut data = vec![2, 4, 4, 5, 99, 0];
        Opcode::process(&mut data)?;

        assert_eq!(vec![2, 4, 4, 5, 99, 9_801], data);
        Ok(())
    }

    #[test]
    fn process_4() -> Result<(), &'static str> {
        let mut data = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let pos0 = Opcode::process(&mut data)?;

        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], data);
        assert_eq!(30, pos0);
        Ok(())
    }

    #[test]
    fn restore_gravity_inplace() {
        let mut data = vec![0, 0, 0, 0];
        restore_gravity(&mut data);

        assert_eq!(vec![0, 12, 2, 0], data);
    }

    #[test]
    fn process_day2_part1() -> Result<(), &'static str> {
        let mut data = vec![
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 9, 1, 19, 1, 19, 5, 23, 1, 23, 6,
            27, 2, 9, 27, 31, 1, 5, 31, 35, 1, 35, 10, 39, 1, 39, 10, 43, 2, 43, 9, 47, 1, 6, 47,
            51, 2, 51, 6, 55, 1, 5, 55, 59, 2, 59, 10, 63, 1, 9, 63, 67, 1, 9, 67, 71, 2, 71, 6,
            75, 1, 5, 75, 79, 1, 5, 79, 83, 1, 9, 83, 87, 2, 87, 10, 91, 2, 10, 91, 95, 1, 95, 9,
            99, 2, 99, 9, 103, 2, 10, 103, 107, 2, 9, 107, 111, 1, 111, 5, 115, 1, 115, 2, 119, 1,
            119, 6, 0, 99, 2, 0, 14, 0,
        ];
        restore_gravity(&mut data);
        assert_eq!(Opcode::process(&mut data)?, 3516593);
        Ok(())
    }
}
