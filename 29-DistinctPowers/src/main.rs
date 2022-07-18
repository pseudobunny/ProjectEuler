use std::collections::HashSet;
use num_bigint::ToBigUint;

fn main() {
    let mut d_powers = HashSet::new();

    for a in 2..=100 {
        for b in 2..=100 {
            d_powers.insert(a.to_biguint().expect("").pow(b));
        }
    }

    println!("{}", d_powers.len())
}