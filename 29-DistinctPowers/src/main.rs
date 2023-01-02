use num::BigUint;
use std::collections::HashSet;

fn num_distinct_powers(max_a: u32, max_b: u32) -> usize {
    let d_powers = (2..=max_a)
        .map(|a| BigUint::from(a))
        .flat_map(|a| (2..=max_b).map(|b| a.pow(b)).collect::<Vec<BigUint>>());

    HashSet::<BigUint>::from_iter(d_powers).len()
}

fn main() {
    println!("{}", num_distinct_powers(100, 100))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(num_distinct_powers(5, 5), 15)
    }

    #[test]
    fn q_case() {
        assert_eq!(num_distinct_powers(100, 100), 9183)
    }
}
