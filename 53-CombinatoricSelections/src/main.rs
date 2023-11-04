use memoize::memoize;
use num::BigUint;

#[memoize]
fn memoized_factorial(n: BigUint) -> BigUint {
    if &n == &BigUint::from(0_u64) {
        BigUint::from(1_u64)
    } else {
        &n * memoized_factorial(&n - 1_u64)
    }
}

fn factorial(n: &BigUint) -> BigUint {
    memoized_factorial(n.clone())
}

fn combination(n: &BigUint, r: &BigUint) -> BigUint {
    factorial(n) / (factorial(r) * factorial(&(n - r)))
}

fn combinatoric_values_for_all_valid_r(n: u64) -> Vec<BigUint> {
    (1..=n)
        .map(|r| combination(&BigUint::from(n), &BigUint::from(r)))
        .collect()
}

fn number_of_combinatoric_values_greater_than(max: u64, max_n: u64) -> usize {
    (1..=max_n)
        .flat_map(|n| combinatoric_values_for_all_valid_r(n))
        .filter(|c| *c > BigUint::from(max))
        .count()
}

fn main() {
    println!(
        "{}",
        number_of_combinatoric_values_greater_than(1_000_000, 100)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(
            number_of_combinatoric_values_greater_than(1_000_000, 100),
            4075
        );
    }
}
