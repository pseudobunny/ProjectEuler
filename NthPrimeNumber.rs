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

fn main() {
    let mut counter = 0;
    let mut curr_n = 1;

    let prime_to_find = 10_001;

    while counter < prime_to_find {
        curr_n += 1;
        if check_primality(curr_n) {
            counter += 1;
        }
    }

    println!("{}", curr_n);
}