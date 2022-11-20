use num_bigint::{ ToBigUint, BigUint };
use num_traits::{ Zero, One };

fn digit_to_vec(num: BigUint) -> Vec<BigUint> {
    fn push_inner(n: BigUint, digits: &mut Vec<BigUint>) {
        digits.push(n.clone() % 10_u64);

        if n >= 10.to_biguint().unwrap() {
            push_inner(n/10_u64, digits);
        }
    }
    
    let mut digits: Vec<BigUint> = vec![];
    push_inner(num, &mut digits);

    digits.into_iter().rev().collect()
}

fn slice_to_int(n_a: &[BigUint]) -> BigUint {
    let mut t: BigUint = One::one();
    let mut n: BigUint = Zero::zero();
    for n_d in n_a.iter().rev() {
        n += n_d.clone()*t.clone();
        t *= 10_u64;
    }

    n
}

fn reverse_digit(num: BigUint) -> BigUint {
    let digit_vec = digit_to_vec(num);
    slice_to_int(&(digit_vec.iter().rev().cloned().collect::<Vec<BigUint>>())[..])
}

fn reverse_sum(num: BigUint) -> BigUint {
    num.clone() + reverse_digit(num)
}

fn check_palindrome(x: BigUint) -> bool {
    let x_str = x.to_string();
    let n = x_str.len() / 2;

    x_str.bytes().take(n).eq(x_str.bytes().rev().take(n))
}

fn is_lychrel(num: BigUint) -> bool {
    let max_iterations = 50;

    let mut curr_num = num;
    for _ in 0..max_iterations {
        curr_num = reverse_sum(curr_num.clone());

        if check_palindrome(curr_num.clone()) {
            return false;
        }
    }

    true
}

fn main() {
    let lychrels = (1..10_000).map(|n| n.to_biguint().unwrap())
        .filter(|n| is_lychrel(n.clone()));

    println!("{}", lychrels.count());
}
