use custom_math_utilities::check_primality;

fn prime_chain_length(a: i64, b: i64) -> i64 {
    let generator = |n: i64| -> i64 { n * n + a * n + b };

    for n in 0.. {
        if !check_primality(generator(n)) {
            return n;
        }
    }

    0
}

fn max_prime_chain() -> (i64, i64) {
    let a = (1..1000).rev().map(|i| -i).chain(0..1000);
    let b = (1..=1000).rev().map(|i| -i).chain(0..=1000);

    a.flat_map(|a_n| b.clone().map(|b_n| (a_n, b_n)).collect::<Vec<(i64, i64)>>())
        .map(|(a_n, b_n)| ((a_n, b_n), prime_chain_length(a_n, b_n)))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0
}

fn main() {
    let result = max_prime_chain();
    println!("{}", result.0 * result.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        let result = max_prime_chain();
        assert_eq!(result.0 * result.1, -59231)
    }
}
