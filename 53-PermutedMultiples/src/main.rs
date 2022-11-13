use num_bigint::{ ToBigUint, BigUint };
use num_traits::{ Zero, One };

fn big_uint_to_digits(n: BigUint) -> Vec<BigUint> {
    let mut digits = vec![];
    let mut n = n;
    let biguint_ten = 10.to_biguint().unwrap();

    while n > Zero::zero() {
        digits.push(n.clone() % biguint_ten.clone());
        n /= biguint_ten.clone();
    }

    digits.sort();

    digits
}

fn main() {
    let big_one: BigUint = One::one();
    let mut i: BigUint = big_one.clone();
    let multipliers = (1..=6).map(|n| n.to_biguint().unwrap());

    loop {
        let multiples_digits = multipliers.clone()
            .map(|n| n*i.clone())
            .map(big_uint_to_digits)
            .collect::<Vec<Vec<BigUint>>>();

        let all_digits_the_same = multiples_digits.windows(2)
            .all(|ab| ab[0]==ab[1]);

        if all_digits_the_same {
            break;
        }

        i += big_one.clone();
    }

    println!("{}", i)
}
