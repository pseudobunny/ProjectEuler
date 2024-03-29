use custom_math_utilities::{big_num_to_digits, check_palindrome, digits_to_big_num};
use num::BigUint;

fn reverse_digit(num: &BigUint) -> BigUint {
    let digit_vec = big_num_to_digits(num.clone());
    let reversed_vec = digit_vec.iter().rev().cloned().collect::<Vec<BigUint>>();
    digits_to_big_num(&reversed_vec)
}

fn reverse_sum(num: &BigUint) -> BigUint {
    num + reverse_digit(num)
}

fn is_lychrel(num: BigUint, max_iterations: usize) -> bool {
    let mut curr_num = num;
    for _ in 0..max_iterations {
        curr_num = reverse_sum(&curr_num);

        if check_palindrome(&curr_num) {
            return false;
        }
    }

    true
}

fn lychrel_numbers_below(max: u64, max_iterations: usize) -> usize {
    (1..max)
        .filter(|&n| is_lychrel(BigUint::from(n), max_iterations))
        .count()
}

fn main() {
    println!("{}", lychrel_numbers_below(10_000, 50));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(lychrel_numbers_below(10_000, 50), 249);
    }
}
