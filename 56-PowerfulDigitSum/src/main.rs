use num::BigUint;
use num::pow::pow;

fn digit_sum(num: BigUint) -> BigUint {
    fn sum_inner(n: BigUint, sum: &mut BigUint) {
        *sum += n.clone() % 10_u64;

        if n >= BigUint::from(10_u64) {
            sum_inner(n/10_u64, sum);
        }
    }
    
    let mut sum: BigUint = BigUint::from(0_u64);
    sum_inner(num, &mut sum);

    sum
}

fn main() {
    let powerful_digits = (1..100).map(|a| BigUint::from(a as u64))
        .map(|a| (1..100).map(|b| pow(a.clone(), b)).map(digit_sum).collect::<Vec<BigUint>>())
        .collect::<Vec<Vec<BigUint>>>()
        .concat();

    println!("{}", powerful_digits.iter().max().unwrap())
}
