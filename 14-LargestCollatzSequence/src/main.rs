fn collatz_sequence_inner(start_num: u64, step: u64) -> u64 {
    let next_step = step + 1;
    let next_num: u64;

    next_num = if start_num % 2 == 0 {
        start_num / 2
    } else {
        start_num * 3 + 1
    };

    if next_num == 1 {
        next_step
    } else {
        collatz_sequence_inner(next_num, next_step)
    }
}

fn collatz_sequence(start_num: u64) -> u64 {
    collatz_sequence_inner(start_num, 0)
}

fn largest_collatz_sequence(max_start: u64) -> u64 {
    (1..max_start)
        .map(|i| (i, collatz_sequence(i)))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0
}

fn main() {
    println!("{}", largest_collatz_sequence(1_000_001))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(largest_collatz_sequence(1_000_001), 837799)
    }
}
