use custom_math_utilities::prime_partition;

fn first_to_sum_greater_than(max: u64) -> Option<u64> {
    (4..)
        .find(|&n| prime_partition(n) > max)
}

fn main() {
    println!("{}", first_to_sum_greater_than(5000).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(first_to_sum_greater_than(4), Some(10))
    }

    #[test]
    fn q_case() {
        assert_eq!(first_to_sum_greater_than(5000), Some(71))
    }
}
