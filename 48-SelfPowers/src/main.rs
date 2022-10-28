use num_bigint::{BigUint, ToBigUint};
use num_traits::Zero;

fn main() {
    let mut sum: BigUint = Zero::zero();

    for i in 1..=1000 {
        let ib = i.to_biguint().unwrap();
        sum += ib.pow(i);
    }

    // full big number
    let s = format!("{}", sum);
    // Get the last ten characters
    let sub_s = s.chars()
                 .skip(s.len()-10)
                 .take(10)
                 .collect::<String>();

    println!("{}", sub_s) 
}
