use num::BigUint;

fn last_ten_digits_of_d_to_pow_d_sum(max: u32) -> String {
    let sum: BigUint = (1..=max).map(|i| BigUint::from(i).pow(i)).sum();

    // full big number
    let s = format!("{}", sum);
    // Get the last ten characters
    s.chars().skip(s.len() - 10).take(10).collect::<String>()
}

fn main() {
    println!("{}", last_ten_digits_of_d_to_pow_d_sum(1000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(
            last_ten_digits_of_d_to_pow_d_sum(10),
            String::from("0405071317")
        );
    }

    #[test]
    fn q_case() {
        assert_eq!(
            last_ten_digits_of_d_to_pow_d_sum(1000),
            String::from("9110846700")
        );
    }
}
