use custom_math_utilities::check_primality;
use num::BigUint;

fn get_order(p: u64) -> u64 {
    let mut k = 1;
    let big_uint_ten = BigUint::from(10_u64);
    let mut curr_mult = BigUint::from(10_u64);

    let mut r;
    loop {
        r = &curr_mult % p;

        if r == BigUint::from(1_u64) {
            break;
        }

        curr_mult *= &big_uint_ten;
        k += 1;
    }

    k
}

fn denom_with_longest_recurring_cycle() -> u64 {
    (95..1000)
        .filter(|&d| check_primality(d))
        .map(|d| (get_order(d), d))
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap()
        .1
}

fn main() {
    println!("{}", denom_with_longest_recurring_cycle())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(denom_with_longest_recurring_cycle(), 983)
    }
}

