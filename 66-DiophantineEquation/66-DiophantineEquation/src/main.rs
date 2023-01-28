use num::{BigInt, Signed, iter::range};

fn chakravala(a: BigInt, b: BigInt, k: BigInt, d: BigInt) -> (BigInt, BigInt) {
    let (new_a, new_b, new_k) = next_triple(
        a.clone(),
        b.clone(),
        k.clone(),
        find_min_m(a, b, k, d.clone()),
        d.clone(),
    );

    if new_k != BigInt::from(1) {
        return chakravala(new_a, new_b, new_k, d);
    }

    (new_a, new_b)
}

fn next_triple(a: BigInt, b: BigInt, k: BigInt, m: BigInt, d: BigInt) -> (BigInt, BigInt, BigInt) {
    let new_a = (a.clone() * m.clone() + d.clone() * b.clone()) / k.abs();
    let new_b = (a + b * m.clone()) / k.abs();
    let new_k = (m.clone() * m - d) / k;

    (new_a, new_b, new_k)
}

fn find_min_m(a: BigInt, b: BigInt, k: BigInt, d: BigInt) -> BigInt {
    // Find the starting m
    let mut start_m = BigInt::from(0);
    for i in range(BigInt::from(0), k.abs()) {
        if (a.clone() + b.clone() * i.clone()) % k.clone() == BigInt::from(0) {
            start_m = i.clone();
        }
    }

    // Define search space
    let m_to_search = (0..10)
        .map(|i| BigInt::from(i as i64))
        .map(|i| start_m.clone() + (i * k.clone()).abs());

    // Find the minimum
    let mut least_m = start_m.clone();
    for m in m_to_search {
        if (m.clone() * m.clone() - d.clone()).abs()
            < (least_m.clone() * least_m.clone() - d.clone()).abs()
        {
            least_m = m;
        }
    }

    least_m
}

fn is_square(n: u64) -> bool {
    let n_sqrt = (n as f64).sqrt();

    (n_sqrt - n_sqrt.trunc()).abs() < 5e-5
}

// Just to make the call in the map nicer
fn chakravala_starter(n: BigInt) -> (BigInt, BigInt) {
    chakravala(BigInt::from(1), BigInt::from(1), 1 - n.clone(), n)
}

fn max_x_of_solutions_under(max: u64) -> BigInt {
    let non_squares: Vec<u64> = (2..=max).filter(|&n| !is_square(n)).collect();

    non_squares
        .iter()
        .map(|&i| BigInt::from(i))
        .map(|n| (n.clone(), chakravala_starter(n).0))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0
}

fn main() {
    println!("{}", max_x_of_solutions_under(1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(max_x_of_solutions_under(1000), BigInt::from(661));
    }
}
