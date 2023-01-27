use core::iter::Sum;
use num::{traits::Pow, Num, NumCast, BigUint};

pub fn num_to_digits<N: Num + NumCast + PartialOrd + Copy>(num: N) -> Vec<N> {
    let ten: N = NumCast::from(10).unwrap();

    fn ntd_inner<N: Num + NumCast + PartialOrd + Copy>(n: N, digits: &mut Vec<N>, ten_inner: N) {
        digits.push(n % ten_inner);
        if n >= ten_inner {
            ntd_inner(n / ten_inner, digits, ten_inner);
        }
    }

    let mut digits = vec![];
    ntd_inner(num, &mut digits, ten);

    digits
}

pub fn digits_to_num<N: Num + NumCast + PartialOrd + Copy + Sum>(digits: &[N]) -> N {
    digits
        .iter()
        .enumerate()
        .map(|(i, n)| *n * NumCast::from(10.pow(i as u32)).unwrap())
        .sum()
}

pub fn digits_to_num_filter_ind<N: Num + NumCast + PartialOrd + Copy + Sum>(
    digits: &[N],
    ind: usize,
) -> N {
    digits_to_num(
        &digits
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != ind)
            .map(|(_, n)| *n)
            .collect::<Vec<N>>(),
    )
}

// BigUint Versions...

pub fn big_num_to_digits(num: BigUint) -> Vec<BigUint> {
    let ten = BigUint::from(10_u32);

    fn ntd_inner(n: BigUint, digits: &mut Vec<BigUint>, ten_inner: BigUint) {
        digits.push(n.clone() % ten_inner.clone());
        if n.clone() >= ten_inner.clone() {
            ntd_inner(n / ten_inner.clone(), digits, ten_inner);
        }
    }

    let mut digits = vec![];
    ntd_inner(num, &mut digits, ten);

    digits
}

pub fn digits_to_big_num(digits: &[BigUint]) -> BigUint {
    digits
        .iter()
        .enumerate()
        .map(|(i, n)| n.clone() * BigUint::from(10_u32).pow(i as u32))
        .sum()
}