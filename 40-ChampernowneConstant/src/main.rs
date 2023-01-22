use custom_math_utilities::num_to_digits;

// This is going to be a very brute force solution...
// There's probably a smart way to count / figure this out, but the brute
// force is so easy with the tools I've built for previous problems.

fn champernowne_constant() -> u64 {
    let mut collect: Vec<u64> = vec![];

    let mut n = 1;
    while collect.len() < 1_000_000 {
        let mut d_vec = num_to_digits(n).into_iter().rev().collect();
        collect.append(&mut d_vec);
        n += 1;
    }

    let mut p = 1;
    let mut t = 1;
    for _ in 0..7 {
        p *= collect[t - 1];
        t *= 10;
    }

    p
}

fn main() {
    println!("{}", champernowne_constant())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(champernowne_constant(), 210);
    }
}
