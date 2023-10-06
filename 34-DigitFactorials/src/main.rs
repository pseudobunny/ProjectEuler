use custom_math_utilities::digit_factorial;

fn is_factorial_digit(num: u64) -> bool {
    digit_factorial(num) == num
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
