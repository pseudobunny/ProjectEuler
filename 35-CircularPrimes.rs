fn num_to_digits(num: u64) -> Vec<u64> {
    fn ntd_inner(n: u64, digits: &mut Vec<u64>) { 
        digits.push(n % 10);
        if n >= 10 {
            ntd_inner(n/10, digits);
        }
    }

    let mut digits = vec![];
    ntd_inner(num, &mut digits);

    digits
}

fn digits_to_num(digits: &Vec<u64>) -> u64 {
    digits.iter()
          .enumerate()
          .map(|(i, n)| n * 10u64.pow(i as u32))
          .sum()
}

fn check_primality(n: u64) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n <= 1 || n % 2 == 0 || n %3 == 0 {
        return false;
    }
    let mut d = 5;
    while d*d <= n {
        if n % d == 0 || n % (d + 2) == 0 {
            return false;
        }
        d += 6;
    }

    return true;
}

fn check_primality_all_rotations(n: u64) -> bool {
    if !check_primality(n) {
        return false;
    }

    let mut digits = num_to_digits(n);
    for _ in 0..(digits.len()-1) {
        digits.rotate_right(1);
        if !check_primality(digits_to_num(&digits)) {
            return false;
        } 
    }

    return true;
}

fn main() {
    let mut count = 13;
    
    for i in 100..1_000_000 {
        if check_primality_all_rotations(i) {
            count += 1;
        }
    }

    println!("{}", count)
}
