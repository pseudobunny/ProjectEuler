use itertools::Itertools;

const DIVISORS: [u64; 7] = [2, 3, 5, 7, 11, 13, 17];
const DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn check_substring_divisibility(num: &str) -> bool {
    num.chars()
        .collect::<Vec<char>>()
        .windows(3)
        .skip(1)
        .map(|vc| vc.iter().collect::<String>())
        .map(|s| s.parse::<u64>().expect("Could not parse num"))
        .zip(DIVISORS)
        .all(|(s_n, d)| s_n % d == 0)
}

fn sum_of_all_substring_divisible() -> u64 {
    DIGITS
        .iter()
        .copied()
        .permutations(10)
        .unique()
        .map(|p| p.join(""))
        .filter(|p| check_substring_divisibility(p))
        .map(|p| p.parse::<u64>().expect("Could not parse num"))
        .sum()
}

fn main() {
    println!("{}", sum_of_all_substring_divisible());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(sum_of_all_substring_divisible(), 16695334890);
    }
}
