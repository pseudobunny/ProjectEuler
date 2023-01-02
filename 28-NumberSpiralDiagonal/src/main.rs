fn sum_of_diagonals_up_to_layer(layer: u64) -> u64 {
    (1..=layer)
        .flat_map(|i| vec![2 * i; 4])
        .fold((1, 1), |(sum, prev_num), n| {
            (sum + prev_num + n, prev_num + n)
        })
        .0
}

fn main() {
    println!("{}", sum_of_diagonals_up_to_layer(500))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(sum_of_diagonals_up_to_layer(2), 101)
    }

    #[test]
    fn q_case() {
        assert_eq!(sum_of_diagonals_up_to_layer(500), 669171001)
    }
}

