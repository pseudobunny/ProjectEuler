use custom_math_utilities::num_to_digits;

static FACTORIAL: [u64; 10] = [
    1,
    1,
    2,
    6,
    24,
    120,
    720,
    5_040,
    40_320,
    362_880
];

fn sum_factorial_digits(num: u64) -> u64 {
    num_to_digits(num)
        .iter()
        .map(|&n| FACTORIAL[n as usize])
        .sum()
}

fn is_factorial_digit(num: u64) -> bool {
    sum_factorial_digits(num) == num
}

fn sum_all_numbers_that_are_factorial_digits() -> u64 {
    (3..1_000_000).filter(|n| is_factorial_digit(*n)).sum()
}

fn main() {
    println!("{}", sum_all_numbers_that_are_factorial_digits())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(sum_all_numbers_that_are_factorial_digits(), 40730)
    }
}
