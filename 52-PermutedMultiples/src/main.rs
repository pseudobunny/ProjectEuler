use custom_math_utilities::num_to_digits;

fn create_multiples_digits(i: u64, mult_max: u64) -> Vec<Vec<u64>> {
    (1..=mult_max)
        .map(|n| n * i)
        .map(|n| {
            let mut digits = num_to_digits(n);
            digits.sort();
            digits
        })
        .collect()
}

fn permuted_multiples(mult_max: u64) -> u64 {
    (1..)
        .map(|i| (i, create_multiples_digits(i, mult_max)))
        .find(|(_, multiples_digits)| multiples_digits.windows(2).all(|ab| ab[0] == ab[1]))
        .unwrap()
        .0
}

fn main() {
    println!("{}", permuted_multiples(6))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(permuted_multiples(6), 142857);
    }
}
