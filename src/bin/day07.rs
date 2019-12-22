extern crate adventofcode2019;
extern crate permutohedron;

use adventofcode2019::intcode;
use permutohedron::LexicalPermutation;

fn program_amplifiers(phase_signals: &Vec<i32>, program: &intcode::Intcode) -> i32 {
    assert_eq!(phase_signals.len(), 5);

    let mut program_a = program.clone();
    let mut program_b = program.clone();
    let mut program_c = program.clone();
    let mut program_d = program.clone();
    let mut program_e = program.clone();

    let outputs_a = program_a.run(vec![phase_signals[0], 0]);
    assert_eq!(outputs_a.len(), 1);

    let ouputs_b = program_b.run(vec![phase_signals[1], outputs_a[0]]);
    assert_eq!(ouputs_b.len(), 1);

    let outputs_c = program_c.run(vec![phase_signals[2], ouputs_b[0]]);
    assert_eq!(outputs_c.len(), 1);

    let outputs_d = program_d.run(vec![phase_signals[3], outputs_c[0]]);
    assert_eq!(outputs_d.len(), 1);

    let outputs_e = program_e.run(vec![phase_signals[4], outputs_d[0]]);
    assert_eq!(outputs_e.len(), 1);

    outputs_e[0]
}

struct Permutator {
    data: Vec<i32>,
    permutated: bool,
}

impl Permutator {
    fn new(data: Vec<i32>) -> Permutator {
        Permutator {
            data,
            permutated: false,
        }
    }
}

impl Iterator for Permutator {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Vec<i32>> {
        if !self.permutated {
            self.permutated = true;
            return Some(self.data.clone());
        }

        if self.data.next_permutation() {
            return Some(self.data.clone());
        }

        None
    }
}

fn generate_phase_signals() -> Permutator {
    Permutator::new(vec![0, 1, 2, 3, 4])
}

fn find_max_amplification(program: &intcode::Intcode) -> i32 {
    generate_phase_signals()
        .map(|signals| program_amplifiers(&signals, program))
        .max()
        .expect("Failed to compute max amplification!")
}

fn main() {
    let program = intcode::Intcode::from_file("inputs/day07.txt");
    println!("Max Signal: {}", find_max_amplification(&program));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_amplifiers_1() {
        let signals = vec![4, 3, 2, 1, 0];
        let program = intcode::Intcode::new(vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ]);
        assert_eq!(program_amplifiers(&signals, &program), 43210);
    }

    #[test]
    fn max_amplification_1() {
        let program = intcode::Intcode::new(vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ]);
        assert_eq!(find_max_amplification(&program), 43210);
    }

    #[test]
    fn max_amplification_2() {
        let program = intcode::Intcode::new(vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ]);
        assert_eq!(find_max_amplification(&program), 54321);
    }

    #[test]
    fn permutate() {
        let mut perm = Permutator::new(vec![1, 2, 3]);

        assert_eq!(perm.next(), Some(vec![1, 2, 3]));
        assert_eq!(perm.next(), Some(vec![1, 3, 2]));
        assert_eq!(perm.next(), Some(vec![2, 1, 3]));
        assert_eq!(perm.next(), Some(vec![2, 3, 1]));
        assert_eq!(perm.next(), Some(vec![3, 1, 2]));
        assert_eq!(perm.next(), Some(vec![3, 2, 1]));
        assert_eq!(perm.next(), None);
    }
}
