fn sum_of_squares(start: u32, end: u32) -> u32 {
    let mut sum = 0;
    for i in start..=end {
        sum += i * i;
    }
    sum
}

fn square_of_sum(start: u32, end: u32) -> u32 {
    let mut sum = 0;
    for i in start..=end {
        sum += i;
    }
    sum * sum
}

fn sum_square_difference(n: u32) -> u32 {
    square_of_sum(1, n) - sum_of_squares(1, n)
}

fn main() {
    println!("{}", sum_square_difference(100))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(sum_square_difference(10), 2640)
    }

    #[test]
    fn q_case() {
        assert_eq!(sum_square_difference(100), 25164150)
    }
}
