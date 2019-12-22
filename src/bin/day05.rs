extern crate adventofcode2019;
use adventofcode2019::intcode;

fn main() {
    println!("Repair board computer:");
    let out = intcode::Intcode::from_file("inputs/day05.txt").run(vec![1]);
    println!("{:?}", out);

    println!();
    println!("Turn on heating:");
    let out = intcode::Intcode::from_file("inputs/day05.txt").run(vec![5]);
    println!("{:?}", out);
}
