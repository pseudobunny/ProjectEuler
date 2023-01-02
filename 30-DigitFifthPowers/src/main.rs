fn sum_pow_digits(num: u64, p: u32) -> u64 {
    fn sum_inner(n: u64, sum: &mut u64, p: u32) {
        *sum += (n % 10).pow(p);
        if n >= 10 {
            sum_inner(n / 10, sum, p);
        }
    }

    let mut sum = 0;
    sum_inner(num, &mut sum, p);

    sum
}

fn sum_nums_that_are_sum_of_pow_digits(p: u32) -> u64 {
    let max = vec![9 as u64; p as usize].iter().map(|n| n.pow(p)).sum();

    (2..=max).filter(|&i| i == sum_pow_digits(i, p)).sum()
}

fn main() {
    println!("{}", sum_nums_that_are_sum_of_pow_digits(5))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(sum_nums_that_are_sum_of_pow_digits(4), 19316)
    }

    #[test]
    fn q_case() {
        assert_eq!(sum_nums_that_are_sum_of_pow_digits(5), 443839)
    }
}
