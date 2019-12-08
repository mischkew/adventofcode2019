fn to_digits(mut number: u32, digit_count: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();

    for exp in (0..digit_count).rev() {
        let current_position = 10_u32.pow(exp);
        let digit = number / current_position;
        assert!(digit < 10, "Digit was {}", digit);

        digits.push(digit);
        number = number - digit * current_position;
    }

    digits
}

fn has_pair_digits(digits: &Vec<u32>) -> bool {
    digits
        .iter()
        .zip(digits.iter().skip(1))
        .any(|(a, b)| a == b)
}

fn has_exact_pair_digits(digits: &Vec<u32>) -> bool {
    assert_eq!(digits.len(), 6);

    let is_exact_pair = |pos| {
        digits[pos - 1] != digits[pos]
            && digits[pos] == digits[pos + 1]
            && digits[pos + 1] != digits[pos + 2]
    };

    let pair1 = digits[0] == digits[1] && digits[1] != digits[2];
    let pair5 = digits[4] == digits[5] && digits[4] != digits[3];

    let pair2 = is_exact_pair(1);
    let pair3 = is_exact_pair(2);
    let pair4 = is_exact_pair(3);

    pair1 || pair2 || pair3 || pair4 || pair5
}

fn has_increasing_digits(digits: &Vec<u32>) -> bool {
    let mut previous = &0;

    for digit in digits.iter() {
        if digit < previous {
            return false;
        }
        previous = digit;
    }

    true
}

fn count_matching_passwords(lower: u32, upper: u32) -> u32 {
    (lower..=upper)
        .map(|number| to_digits(number, 6))
        .filter(|digits| has_pair_digits(digits)) // need to use closure, due to type coercion
        .filter(|digits| has_increasing_digits(digits))
        .count() as u32
}

fn count_matching_passwords_with_exact_pairs(lower: u32, upper: u32) -> u32 {
    (lower..=upper)
        .map(|number| to_digits(number, 6))
        .filter(|digits| has_exact_pair_digits(digits))
        .filter(|digits| has_increasing_digits(digits))
        .count() as u32
}

fn main() {
    let count = count_matching_passwords(172930, 683082);
    println!("Number of possible password: {}", count);

    let count2 = count_matching_passwords_with_exact_pairs(172830, 683082);
    println!("Number of possible password - exact pairs: {}", count2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_digits() {
        assert_eq!(to_digits(100, 3), vec![1, 0, 0]);
        assert_eq!(to_digits(12345, 5), vec![1, 2, 3, 4, 5]);
        assert_eq!(to_digits(0, 1), vec![0]);
        assert_eq!(to_digits(1703001, 7), vec![1, 7, 0, 3, 0, 0, 1]);
    }

    #[test]
    fn should_find_pairs() {
        assert!(has_pair_digits(&vec![1, 0, 0]));
        assert!(has_pair_digits(&vec![1, 1, 0]));
        assert!(has_pair_digits(&vec![1, 2, 3, 4, 5, 5, 6, 7]));
        assert!(!has_pair_digits(&vec![1, 0]));
        assert!(!has_pair_digits(&vec![1]));
        assert!(!has_pair_digits(&vec![1, 2, 3, 4]));
    }

    #[test]
    fn should_find_increasing_digits() {
        assert!(has_increasing_digits(&vec![1, 2, 2, 3]));
        assert!(has_increasing_digits(&vec![2, 2, 2, 2]));
        assert!(has_increasing_digits(&vec![1]));
        assert!(!has_increasing_digits(&vec![1, 2, 2, 3, 0]));
    }

    #[test]
    fn should_find_exact_pairs() {
        assert!(has_exact_pair_digits(&vec![1, 1, 1, 1, 2, 2]));
        assert!(!has_exact_pair_digits(&vec![1, 1, 1, 2, 2, 2]));
        assert!(has_exact_pair_digits(&vec![1, 2, 3, 4, 2, 2]));
        assert!(!has_exact_pair_digits(&vec![1, 2, 3, 4, 5, 6]));
    }
}
