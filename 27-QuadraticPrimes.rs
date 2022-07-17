fn check_primality(n: i64) -> bool {
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
    let mut max_a = 0;
    let mut max_b = 0;
    let mut max_chain = 0;

    for a in -999..=999 {
        for b in -1000..=1000 {
            let generator = |n: i64| -> i64 {
                n*n + a*n + b
            };
            
            for n in 0.. {
                if !check_primality(generator(n)) {
                    if n > max_chain {
                        max_a = a;
                        max_b = b;
                        max_chain = n;
                    }

                    break;
                }                
            }
        }
    }

    println!("{}", max_a*max_b)
}