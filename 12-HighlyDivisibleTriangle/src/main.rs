use custom_math_utilities::triangle_number_list;

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
    let mut tri_num_list = triangle_number_list();

    (1..)
        .map(|i| tri_num_list.get(i))
        .find(|&tri| calc_divisor_num(tri) > n)
        .unwrap()
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
