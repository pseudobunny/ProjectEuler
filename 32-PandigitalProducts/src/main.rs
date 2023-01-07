use itertools::Itertools;
use std::collections::HashSet;

fn slice_to_int(n_a: &[&i64]) -> i64 {
    let mut t = 1;
    let mut n = 0;
    for n_d in n_a.iter().rev() {
        n += *n_d * t;
        t *= 10;
    }

    n
}

const DIGITS: [i64; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

fn pandigital_products() -> HashSet<i64> {
    let mut prods: HashSet<i64> = HashSet::new();

    let mut prod;
    for perm in DIGITS.iter().permutations(DIGITS.len()).unique() {
        for i in 1..7 {
            for j in i..8 {
                prod = slice_to_int(&perm[0..i]) * slice_to_int(&perm[i..j]);
                if prod == slice_to_int(&perm[j..]) {
                    prods.insert(prod);
                }
            }
        }
    }

    prods
}

fn main() {
    println!("{:?}", pandigital_products().iter().sum::<i64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(pandigital_products().iter().sum::<i64>(), 45228)
    }
}
