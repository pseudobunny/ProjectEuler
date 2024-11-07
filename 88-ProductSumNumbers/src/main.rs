use memoize::memoize;
use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

#[memoize]
fn product_sets(x: usize) -> Vec<Vec<usize>> {
    // base cases
    if x == 1 {
        return vec![];
    } else if x == 4 {
        return vec![vec![2, 2]];
    }

    (2..=x / 2)
        .filter(|i| x % i == 0)
        .flat_map(|i| {
            let b = x / i;
            let mut product_set = vec![vec![i, b]];

            product_set.append(
                &mut product_sets(b)
                    .iter()
                    .map(|subset| {
                        let mut new_subset = subset.clone();
                        new_subset.push(i);
                        new_subset
                    })
                    .collect::<Vec<Vec<usize>>>(),
            );

            product_set
        })
        .collect()
}

fn find_minimal_product_sum_number_sum(k_max: usize) -> usize {
    let mut product_number_sum_map = HashMap::<usize, usize>::new();

    for x in 4..=k_max * 2 {
        product_sets(x)
            .iter()
            .filter(|set| set.iter().sum::<usize>() <= x)
            .for_each(|set| {
                let sum = set.iter().sum::<usize>();
                let k = set.len() + (x - sum);

                if k >= k_max {
                    return;
                }

                let v_opt = product_number_sum_map.get(&k);
                let min_sum = v_opt.map_or_else(|| x, |&v| min(v, x));
                product_number_sum_map.insert(k, min_sum);
            });
    }

    product_number_sum_map
        .values()
        .copied()
        .collect::<HashSet<usize>>()
        .iter()
        .sum()
}

fn main() {
    println!("{}", find_minimal_product_sum_number_sum(12000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(find_minimal_product_sum_number_sum(12), 61);
    }

    #[test]
    fn q_case() {
        assert_eq!(find_minimal_product_sum_number_sum(12000), 7587457);
    }
}
