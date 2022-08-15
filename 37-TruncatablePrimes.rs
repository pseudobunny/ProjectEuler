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

fn check_truncated(n: u64) -> bool {
    let mut n_left = n;
    let mut n_right = n;

    loop {
        if !(check_primality(n_left) && check_primality(n_right)) {
            return false;
        }

        if n_left < 10 {
            break;
        }

        // reduction step
        let pow = (n_left as f64).log10().trunc() as u32;
        let tenpow = 10_u64.pow(pow); 
        n_left = n_left - (n_left / tenpow) * tenpow;
    
        n_right = n_right / 10;
    }

    return true;
}

fn main() {
    let mut sum = 0;
    let mut found = 0;
    let mut i = 10;

    loop {
        if check_truncated(i) {
            println!("{}", i);
            sum += i;
            found += 1;
        }

        if found > 10 {
            break;
        }

        i += 1;
    }

    println!("{}", sum)
}
