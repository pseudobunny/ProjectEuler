use std::collections::HashMap;

use custom_math_utilities::big_num_to_digits;
use num::BigUint;

struct CubePermutation {
    value: BigUint,
    permutation: String,
}

fn construct_cube_permutation(n: u64) -> CubePermutation {
    let n_cube = BigUint::from(n) * BigUint::from(n) * BigUint::from(n);
    let n_str = get_sorted_permutaiton_string(&n_cube);

    CubePermutation {
        value: n_cube,
        permutation: n_str,
    }
}

fn get_sorted_permutaiton_string(n: &BigUint) -> String {
    let mut n_digits = big_num_to_digits(n.clone());

    n_digits.sort();

    n_digits.iter().map(|i| i.to_string()).collect()
}

fn smallest_cube_with_permutations(perm_target: usize) -> Option<BigUint> {
    let mut permutation_map = HashMap::<String, (usize, BigUint)>::new();

    for cube in (0_u64..).map(construct_cube_permutation) {
        let new_value = permutation_map
            .get(&cube.permutation)
            .map_or((1, cube.value.clone()), |(count, min_value)| {
                (count + 1, std::cmp::min(min_value.clone(), cube.value))
            });

        if new_value.0 >= perm_target {
            return Some(new_value.1);
        }

        permutation_map.insert(cube.permutation, new_value);
    }

    None
}

fn main() {
    println!("{}", smallest_cube_with_permutations(5).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(
            smallest_cube_with_permutations(3),
            Some(BigUint::from(41063625_u64))
        )
    }

    #[test]
    fn q_case() {
        assert_eq!(
            smallest_cube_with_permutations(5),
            Some(BigUint::from(127035954683_u64))
        )
    }
}
