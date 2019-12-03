use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn load_input(filename: &str) -> Result<Vec<u32>, Error> {
    let io = File::open(filename)?;
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn fuel_for_module(mass: u32) -> u32 {
    let third = mass / 3;
    if third < 2 {
        0
    } else {
        third - 2
    }
}

fn recursive_fuel_for_module(mass: u32) -> u32 {
    let fuel = fuel_for_module(mass);
    if fuel > 0 {
        return fuel + recursive_fuel_for_module(fuel);
    }
    fuel
}

fn fuel_for_modules(masses: &Vec<u32>) -> u32 {
    masses.iter().map(|m| fuel_for_module(*m)).sum()
}

fn recursive_fuel_for_modules(masses: &Vec<u32>) -> u32 {
    masses.iter().map(|m| recursive_fuel_for_module(*m)).sum()
}

fn main() -> Result<(), Error> {
    let masses = load_input("inputs/day01.txt")?;
    println!(
        "Fuel required for all modules (simple): {}",
        fuel_for_modules(&masses)
    );

    println!(
        "Fuel required for all modules (fuel for fuel): {}",
        recursive_fuel_for_modules(&masses)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_of_12_has_fuel_of_2() {
        assert_eq!(2, fuel_for_module(12));
    }

    #[test]
    fn module_of_14_has_fuel_of_2() {
        assert_eq!(2, fuel_for_module(14));
    }

    #[test]
    fn module_of_1969_has_fuel_of_654() {
        assert_eq!(654, fuel_for_module(1_969));
    }

    #[test]
    fn module_of_100756_has_fuel_of_33583() {
        assert_eq!(33_583, fuel_for_module(100_756));
    }

    #[test]
    fn recursive_fuel_12() {
        assert_eq!(2, recursive_fuel_for_module(12));
    }

    #[test]
    fn recursive_fuel_1969() {
        assert_eq!(966, recursive_fuel_for_module(1_969));
    }

    #[test]
    fn collect_fuel_requirements() {
        assert_eq!(658, fuel_for_modules(&vec![12, 14, 1_969]));
    }

    #[test]
    fn collect_recursive_fuel_requirements() {
        assert_eq!(968, recursive_fuel_for_modules(&vec![14, 1_969]));
    }

    #[test]
    fn borrowing_for_same_input() {
        let input = vec![12, 14, 1_969];
        assert_eq!(658, fuel_for_modules(&input));
        assert_eq!(658, fuel_for_modules(&input));
    }
}
