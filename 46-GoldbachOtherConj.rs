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
    let mut primes = vec![2,3,5,7];
    
    let mut i = 9;
    let mut found = false;

    while !found {
        if check_primality(i) {
            primes.push(i);
            i += 1;
            continue;
        }

        if i % 2 != 0 {
            let mut fits_conj = false;
            for p in &primes {
                if p > &i {
                    break;
                }
                
                let mut j = 1;
                loop {
                    let sum = p + 2*j*j;
                    
                    if sum == i {
                        fits_conj = true;
                        break;
                    }

                    if sum > i {
                        break;
                    }
                    
                    j += 1; 
                }

                if fits_conj {
                    break;
                }
            }

            found = !fits_conj;
        }

        i += 1;
    }
    
    println!("{}", i)
}
