const COINS: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

fn configurations_of_value_from_coins(total: usize) -> u64 {
    let mut configurations = vec![0; total + 1];
    configurations[0] = 1;

    for i in 0..8 {
        for j in COINS[i]..=total {
            configurations[j] += configurations[j - COINS[i]];
        }
    }

    configurations[total]
}

fn main() {
    println!("{}", configurations_of_value_from_coins(200))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(configurations_of_value_from_coins(200), 73682)
    }
}
