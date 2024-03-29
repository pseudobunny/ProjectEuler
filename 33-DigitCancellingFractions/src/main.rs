use gcd::Gcd;
use custom_math_utilities::{digits_to_num_filter_ind, num_to_digits};

fn find_common_digits(n: u64, m: u64) -> Vec<(u64, u64)> {
    let n_digits = num_to_digits(n);
    let m_digits = num_to_digits(m);

    let mut out: Vec<(u64, u64)> = vec![];
    for (i, n_d) in n_digits.iter().enumerate() {
        for (j, m_d) in m_digits.iter().enumerate() {
            if n_d == m_d && (i != 0 || j != 0) {
                out.push((
                    digits_to_num_filter_ind(&n_digits, i),
                    digits_to_num_filter_ind(&m_digits, j),
                ));
            }
        }
    }

    out
}

fn find_reduced_fraction(n: u64, m: u64) -> (u64, u64) {
    let nm_gcd = n.gcd(m);
    (n / nm_gcd, m / nm_gcd)
}

fn find_curious_fraction_product() -> (u64, u64) {
    let mut curious_nums = vec![];
    for i in 11.. {
        for j in 10..i {
            let reduced = find_reduced_fraction(j, i);
            let curious = find_common_digits(j, i)
                .iter()
                .map(|(n, m)| find_reduced_fraction(*n, *m))
                .any(|x| x == reduced);

            if curious {
                curious_nums.push(reduced)
            }
        }
        if curious_nums.len() >= 4 {
            break;
        }
    }

    let all_together = curious_nums
        .iter()
        .fold((1, 1), |acc, x| (acc.0 * x.0, acc.1 * x.1));

    find_reduced_fraction(all_together.0, all_together.1)
}

fn main() {
    println!("{:?}", find_curious_fraction_product())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(find_curious_fraction_product(), (1, 100))
    }
}
