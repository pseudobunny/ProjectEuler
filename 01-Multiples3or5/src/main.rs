fn mul3or5(val: i32) -> bool {
    (val % 3 == 0) || (val % 5 == 0)
}

fn sum_mul_below(n: i32) -> i32 {
    (3..n).filter(|i| mul3or5(*i)).sum()
}

fn main() {
    println!("{}", sum_mul_below(1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(sum_mul_below(10), 23)
    }

    #[test]
    fn q_case() {
        assert_eq!(sum_mul_below(1000), 233168)
    }
}
