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

    true
}

fn main() {
    let mut primes_on_diag = 0_u64;
    let mut diag_numbers = 1_u64;

    let mut prev_layer_end = 1;
    let mut ratio = 1_f64;
    let mut i = 0;
    while ratio > 0.1_f64 { 
        i += 1;
        let layer_width = 2*i + 1;
        let corners = (1..=4_u64).map(|j| (layer_width-1)*j + prev_layer_end);

        diag_numbers += 4;
        primes_on_diag += corners.filter(|c| check_primality(*c)).count() as u64;
        prev_layer_end += layer_width*4 - 4;
    
        ratio = primes_on_diag as f64 / diag_numbers as f64;
    }

    println!("{}", 2*i + 1)
}
