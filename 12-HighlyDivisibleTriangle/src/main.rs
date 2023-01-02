fn gen_triangle_number(n: u64) -> u64 {
    (n * (n + 1)) / 2
}

fn calc_divisor_num(n: u64) -> u64 {
    let mut max: u64 = n / 2;
    let mut divisor_num: u64 = 0;
    let mut i: u64 = 2;
    loop {
        if i >= max {
            break;
        }

        if n % i == 0 {
            max = n / i;
            divisor_num += 2;
        }

        i += 1;
    }

    divisor_num + 2
}

fn triangle_num_with_divisors_over(n: u64) -> u64 {
    let mut i = 1;
    let mut triangle_num: u64;
    loop {
        triangle_num = gen_triangle_number(i);

        if calc_divisor_num(triangle_num) > n {
            break;
        }

        i += 1;
    }

    triangle_num
}

fn main() {
    println!("{}", triangle_num_with_divisors_over(500))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(triangle_num_with_divisors_over(5), 28)
    }

    #[test]
    fn q_case() {
        assert_eq!(triangle_num_with_divisors_over(500), 76576500)
    }
}
