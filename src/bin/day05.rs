extern crate adventofcode2019;
use adventofcode2019::intcode;

fn main() {
    println!("Repair board computer:");
    intcode::Intcode::from_file("inputs/day05.txt").run(1);

    println!();
    println!("Turn on heating:");
    intcode::Intcode::from_file("inputs/day05.txt").run(5);
}
