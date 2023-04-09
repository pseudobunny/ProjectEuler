use custom_math_utilities::{
    hepta_number_list, hexa_number_list, octa_number_list, penta_number_list, square_number_list,
    triangle_number_list, GeneratedFuncList,
};
use itertools::Itertools;

fn check_cyclic_set(set: &[u64]) -> bool {
    set.iter()
        .cycle()
        .take(7)
        .collect_vec()
        .windows(2)
        .all(|pairs| pairs[0] % 100 == pairs[1] / 100)
}

#[derive(Clone)]
struct CyclicFigurateSet {
    used_number_types: Vec<bool>,
    data: Vec<u64>,
}

impl CyclicFigurateSet {
    fn new(set_len: usize, starter_num: u64, starter_list: usize) -> CyclicFigurateSet {
        CyclicFigurateSet {
            used_number_types: (0..set_len).map(|i| i == starter_list).collect_vec(),
            data: vec![starter_num],
        }
    }

    fn extend_set(&self, func_list_values: &Vec<Vec<u64>>) -> Vec<CyclicFigurateSet> {
        let func_list_values_to_check = func_list_values
            .iter()
            .enumerate()
            .filter(|(i, _)| !self.used_number_types[*i])
            .collect_vec();

        let possible_next_nums = find_next_cyclic_numbers(*self.data.last().unwrap());

        let set_extentions = possible_next_nums.iter().flat_map(|num| {
            func_list_values_to_check
                .iter()
                .filter_map(|(i, values)| {
                    if values.contains(num) {
                        Some((i, num))
                    } else {
                        None
                    }
                })
                .collect_vec()
        });

        set_extentions
            .map(|(&i, &num)| {
                let mut new_set = self.clone();
                new_set.data.push(num);
                new_set.used_number_types[i] = true;
                new_set
            })
            .collect_vec()
    }
}

fn find_next_cyclic_numbers(num: u64) -> Vec<u64> {
    let new_base = (num % 100) * 100;

    (0..100).map(|n| new_base + n).collect_vec()
}

fn construct_cyclic_set(
    func_list_values: &Vec<Vec<u64>>,
    set_len: usize,
    starter_num: u64,
    starter_list: usize,
) -> Option<CyclicFigurateSet> {
    let mut sets = vec![CyclicFigurateSet::new(set_len, starter_num, starter_list)];

    while sets.iter().count() != 0 && sets.first().unwrap().data.len() < set_len {
        sets = sets
            .iter()
            .flat_map(|set| set.extend_set(func_list_values))
            .collect();
    }

    sets.iter()
        .find(|set| set.data.len() == set_len && check_cyclic_set(&set.data))
        .cloned()
}

const GENERATED_FUNC_CONSTRUCTORS: [fn() -> GeneratedFuncList; 6] = [
    triangle_number_list,
    square_number_list,
    penta_number_list,
    hexa_number_list,
    hepta_number_list,
    octa_number_list,
];

fn find_cyclic_figurate_set(set_len: usize) -> Option<Vec<u64>> {
    let func_list_values = GENERATED_FUNC_CONSTRUCTORS
        .iter()
        .take(set_len)
        .map(|f| f().values_between(999, 10000))
        .collect_vec();

    func_list_values
        .iter()
        .enumerate()
        .flat_map(|(i, values)| {
            values
                .iter()
                .flat_map(|&num| construct_cyclic_set(&func_list_values, set_len, num, i))
                .collect_vec()
        })
        .map(|set| set.data)
        .next()
}

fn main() {
    println!(
        "{:?}",
        find_cyclic_figurate_set(6).unwrap().iter().sum::<u64>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_cyclic_set() {
        assert!(check_cyclic_set(&[8128, 2882, 8281]))
    }

    #[test]
    fn base_case() {
        assert_eq!(find_cyclic_figurate_set(3), Some(vec![8128, 2882, 8281]))
    }

    #[test]
    fn q_case() {
        assert_eq!(find_cyclic_figurate_set(6).unwrap().iter().sum::<u64>(), 28684)
    }
}
