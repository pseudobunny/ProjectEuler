use custom_math_utilities::partition;

fn number_of_ways_to_sum(n: i64) -> i64 {
    partition(n) - 1
}

fn main() {
    println!("{}", number_of_ways_to_sum(100))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(number_of_ways_to_sum(5), 6)
    }

    #[test]
    fn q_case() {
        assert_eq!(number_of_ways_to_sum(100), 190569291)
    }
}
