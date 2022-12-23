fn update_euler(x: (u32, u32)) -> (u32, u32) {
    (x.1, x.0 + x.1)
}

fn sum_even_below(n: u32) -> u32 {
    let mut fib_pair : (u32, u32) = (1, 2);
    let mut fib_sum : u32 = 2;

    while fib_pair.1 < n {
        fib_pair = update_euler(fib_pair);
        if fib_pair.1 % 2 == 0 {
            fib_sum += fib_pair.1;
        }
    }

    fib_sum
}

fn main() {
    println!("{}", sum_even_below(4_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(sum_even_below(70), 44)
    }

    #[test]
    fn q_case() {
        assert_eq!(sum_even_below(4_000_000), 4613732)
    }
}
