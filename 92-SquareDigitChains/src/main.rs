use custom_math_utilities::digit_square;
use memoize::memoize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Chain {
    One,
    EightyNine,
}

#[memoize]
fn determine_chain(n: usize) -> Chain {
    let mut current_n = n as u64;
    loop {
        match current_n {
            1 => return Chain::One,
            89 => return Chain::EightyNine,
            _ => {
                current_n = digit_square(current_n);
            }
        }
    }
}

fn chains_under(n: usize, chain: Chain) -> usize {
    (1..n).filter(|&i| determine_chain(i) == chain).count()
}

fn main() {
    println!("{}", chains_under(10_000_000, Chain::EightyNine));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(determine_chain(44), Chain::One);
        assert_eq!(determine_chain(85), Chain::EightyNine);
    }

    #[test]
    fn q_case() {
        assert_eq!(chains_under(10_000_000, Chain::EightyNine), 8581146);
    }
}
