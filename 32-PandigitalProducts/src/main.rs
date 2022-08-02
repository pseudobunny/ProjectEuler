use std::collections::HashSet;
use itertools::Itertools;

fn slice_to_int(n_a: &[&i64]) -> i64 {
    let mut t = 1;
    let mut n = 0;
    for n_d in n_a.iter().rev() {
        n += *n_d*t;
        t *= 10;
    }

    n
}

fn main() {
    let digits: Vec<i64> = vec![1,2,3,4,5,6,7,8,9];
    let mut prods: HashSet<i64> = HashSet::new(); 

    let mut prod;
    for perm in digits.iter().permutations(digits.len()).unique() {
        for i in 1..7 {
            for j in i..8 {
                prod = slice_to_int(&perm[0..i]) * slice_to_int(&perm[i..j]);
                if prod == slice_to_int(&perm[j..]) {
                    prods.insert(prod);
                }
            }
        }
    }

    println!("{:?}", prods.iter().sum::<i64>())
}
