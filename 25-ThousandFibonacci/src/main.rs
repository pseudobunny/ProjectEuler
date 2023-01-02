use num::BigUint;
use std::mem::replace;

fn fibonacci_ind_with_more_digits_than(n: usize) -> usize {
    let max = format!("{}{}", "1", "0".repeat(n - 1))
        .parse::<BigUint>()
        .expect("error parsing big int");

    let mut f0 = BigUint::from(0_u64);
    let mut f1 = BigUint::from(1_u64);
    let mut i = 1;
    loop {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
        i += 1;

        if f1 >= max {
            break;
        }
    }

    i
}

fn main() {
    println!("{}", fibonacci_ind_with_more_digits_than(1_000))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base_case() {
        assert_eq!(fibonacci_ind_with_more_digits_than(3), 12)
    }

    #[test]
    fn q_case() {
        assert_eq!(fibonacci_ind_with_more_digits_than(1_000), 4782)
    }
}
