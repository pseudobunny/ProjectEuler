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
    let mut sum : u64 = 0;
    let mut curr_n = 1;

    let max_n = 2_000_000;

    while curr_n < max_n {
        if check_primality(curr_n) {
            sum += curr_n as u64;
        }
        curr_n += 1;
    }

    println!("{}", sum);
}