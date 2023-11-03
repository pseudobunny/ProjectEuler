use custom_math_utilities::{check_if_whole, sqrt_decimal_expansion};

fn decimal_expansion_sum(n: u64, precision: usize) -> u64 {
    let decimal_expansion = sqrt_decimal_expansion(n, precision);

    // no good method on this type to get just the decimal portion... so jank
    decimal_expansion
        .to_string()
        .chars()
        .filter(|c| c.is_numeric())
        .take(100)
        .map(|d_c| d_c.to_digit(10).unwrap_or(0) as u64)
        .sum()
}

fn sum_of_decimal_expansion_sums(max: u64, precision: usize) -> u64 {
    (1..=max)
        .filter(|n| !check_if_whole((*n as f64).sqrt()))
        .map(|n| decimal_expansion_sum(n, precision))
        .sum()
}

fn main() {
    let sum = sum_of_decimal_expansion_sums(100, 100);
    println!("{sum:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(decimal_expansion_sum(2, 100), 475)
    }

    #[test]
    fn q_case() {
        assert_eq!(sum_of_decimal_expansion_sums(100, 100), 40886)
    }
}
