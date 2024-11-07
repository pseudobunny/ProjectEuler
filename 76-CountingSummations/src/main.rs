use custom_math_utilities::PartitionSolver;
use num::BigInt;

fn number_of_ways_to_sum(n: usize) -> BigInt {
    PartitionSolver::new().partition(n) - 1
}

fn main() {
    println!("{}", number_of_ways_to_sum(100))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(number_of_ways_to_sum(5), BigInt::from(6))
    }

    #[test]
    fn q_case() {
        assert_eq!(number_of_ways_to_sum(100), BigInt::from(190569291))
    }
}
