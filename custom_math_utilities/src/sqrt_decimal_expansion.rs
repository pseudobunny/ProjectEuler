use bigdecimal::BigDecimal;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use num::{BigInt, BigUint};

use crate::{digits_to_big_num, digits_to_num, num_to_digits};

pub fn sqrt_decimal_expansion(n: u64, precision: usize) -> BigDecimal {
    let digits = num_to_digits(n);
    let mut carry_downs = digits
        .chunks(2)
        .map(|v| digits_to_num(v))
        .chain(std::iter::repeat(0));

    let (expansion, _) = carry_downs
        .fold_while(
            (vec![], BigUint::from(0_u64)),
            |(expansion, remainder), carry_down| {
                if expansion.len() == precision {
                    Done((expansion, remainder))
                } else {
                    Continue(long_division_step(&remainder, carry_down, &expansion))
                }
            },
        )
        .into_inner();

    BigDecimal::new(
        BigInt::from(expansion_vec_to_big_uint(&expansion)),
        (precision - digits.chunks(2).len()) as i64,
    )
}

fn long_division_step(remainder: &BigUint, carry_down: u64, p: &[u64]) -> (Vec<u64>, BigUint) {
    let c = remainder * BigUint::from(100_u64) + BigUint::from(carry_down);

    let p_num = expansion_vec_to_big_uint(p);

    let x = find_x(&p_num, &c);

    let y = &x * (BigUint::from(20_u64) * &p_num + &x);

    let mut new_p = p.to_vec();
    new_p.push(x.iter_u64_digits().next().unwrap_or(0));

    (new_p, c - y)
}

fn expansion_vec_to_big_uint(p: &[u64]) -> BigUint {
    digits_to_big_num(
        &p.iter()
            .rev()
            .copied()
            .map(BigUint::from)
            .collect::<Vec<BigUint>>(),
    )
}

fn find_x(p: &BigUint, c: &BigUint) -> BigUint {
    let start = if p < &BigUint::from(1_u64) {
        BigUint::from(1_u64)
    } else {
        c / (20_u64 * p)
    };

    if compute_y(&start, p) < *c {
        (0_u64..)
            .map(|i| &start + BigUint::from(i))
            .find(|x_i| compute_y(x_i, p) > *c)
            .unwrap_or(BigUint::from(1_u64))
            - BigUint::from(1_u64)
    } else {
        (0_u64..)
            .map(|i| &start - BigUint::from(i))
            .find(|x_i| compute_y(x_i, p) < *c)
            .unwrap_or(BigUint::from(1_u64))
    }
}

fn compute_y(x: &BigUint, p: &BigUint) -> BigUint {
    x * (BigUint::from(20_u64) * p + x)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn sqrt_2_expansion() {
        assert_eq!(sqrt_decimal_expansion(2, 100), BigDecimal::from_str("1.414213562373095048801688724209698078569671875376948073176679737990732478462107038850387534327641572").unwrap())
    }
}
