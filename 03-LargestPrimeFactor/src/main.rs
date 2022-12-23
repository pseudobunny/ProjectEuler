fn prime_factors(n: u64) -> u64 {
    let mut max_factor : u64 = 0;
    let mut curr_n : u64 = n;
    let mut curr_d : u64 = 2;
    while curr_n > 1 {
        while curr_n % curr_d == 0 {
            max_factor = curr_d;
            curr_n /= curr_d;
        }

        curr_d += 1;
        
        if curr_d*curr_d > n {
            if n > 1 {
                max_factor = n;
            }
            break;
        }
    }

    max_factor
}

fn main() {
    println!("{}", prime_factors(600851475143))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(prime_factors(13195), 29)
    }

    #[test]
    fn q_case() {
        assert_eq!(prime_factors(600851475143), 6857)
    }
}
