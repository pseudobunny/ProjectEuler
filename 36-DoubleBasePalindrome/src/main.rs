use custom_math_utilities::to_base;

fn is_palindrome(n_str: String) -> bool {
    n_str == n_str.chars().rev().collect::<String>()
}

fn is_double_palindrome(n: u32) -> bool {
    is_palindrome(n.to_string()) && is_palindrome(to_base(n, 2))
}

fn sum_double_palindromes_below(n: u32) -> u32 {
    (1..n).filter(|&i| is_double_palindrome(i)).sum()
}

fn main() {
    println!("{}", sum_double_palindromes_below(1_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_palindrome() {
        assert!(is_double_palindrome(585));
    }

    #[test]
    fn test_sum_double_palindrome() {
        assert_eq!(sum_double_palindromes_below(1_000_000), 872187);
    }
}