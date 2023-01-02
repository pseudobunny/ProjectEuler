use num::{BigUint, FromPrimitive};

fn sum_digits(num_str: String) -> u32 {
    num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>()
}

fn sum_digits_two_pow(n: u32) -> u32 {
    sum_digits(BigUint::from_u64(2).unwrap().pow(n).to_string())
}

fn main() {
    println!("{}", sum_digits_two_pow(1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(sum_digits_two_pow(15), 26)
    }

    #[test]
    fn q_case() {
        assert_eq!(sum_digits_two_pow(1000), 1366)
    }
}
