use custom_math_utilities::check_primality;

fn layer_that_achieves_ratio(target: f64) -> u64 {
    let mut primes_on_diag = 0_u64;
    let mut diag_numbers = 1_u64;

    let mut prev_layer_end = 1;
    let mut ratio = 1_f64;
    let mut i = 0;
    while ratio > target {
        i += 1;
        let layer_width = 2 * i + 1;
        let corners = (1..=4_u64).map(|j| (layer_width - 1) * j + prev_layer_end);

        diag_numbers += 4;
        primes_on_diag += corners.filter(|c| check_primality(*c)).count() as u64;
        prev_layer_end += layer_width * 4 - 4;

        ratio = primes_on_diag as f64 / diag_numbers as f64;
    }

    2 * i + 1
}

fn main() {
    println!("{}", layer_that_achieves_ratio(0.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(layer_that_achieves_ratio(0.1), 26241);
    }
}
