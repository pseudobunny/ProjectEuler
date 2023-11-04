use num::pow::pow;
use num::BigUint;

fn digit_sum(num: BigUint) -> BigUint {
    fn sum_inner(n: &BigUint, sum: &mut BigUint) {
        *sum += n % 10_u64;

        if n >= &BigUint::from(10_u64) {
            sum_inner(&(n / 10_u64), sum);
        }
    }

    let mut sum: BigUint = BigUint::from(0_u64);
    sum_inner(&num, &mut sum);

    sum
}

fn max_powerful_digit_sum(max_a: usize, max_b: usize) -> BigUint {
    (1..max_a)
        .map(|a| BigUint::from(a as u64))
        .flat_map(|a| {
            (1..max_b)
                .map(|b| pow(a.clone(), b))
                .map(digit_sum)
                .collect::<Vec<BigUint>>()
        })
        .max()
        .unwrap()
}

fn main() {
    println!("{}", max_powerful_digit_sum(100, 100))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(max_powerful_digit_sum(100, 100), BigUint::from(972_u64));
    }
}
