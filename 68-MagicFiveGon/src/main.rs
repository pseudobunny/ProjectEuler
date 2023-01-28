use itertools::Itertools;
use std::collections::HashSet;

const DIGITS: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

fn determine_possible_solutions() -> Vec<(Vec<u32>, Vec<Vec<u32>>)> {
    let inner = DIGITS.iter().permutations(5).unique();

    let outer: Vec<Vec<Vec<u32>>> = inner
        .clone()
        .map(|i| {
            DIGITS
                .iter()
                .filter(|n| !i.contains(n))
                .map(|&n| n as u32)
                .collect::<Vec<u32>>()
        })
        .map(|nv| nv.iter().copied().permutations(5).unique().collect_vec())
        .collect();

    inner
        .clone()
        .map(|iv| iv.into_iter().cycle().take(6).copied().collect_vec())
        .zip(outer)
        .collect()
}

fn magic_five_gon_ring_max_string() -> u64 {
    let possible_solutions = determine_possible_solutions();
    let solution_sets = possible_solutions.iter().flat_map(|iov| {
        let i = iov.0.clone();
        let ov = iov.1.clone();

        ov.iter()
            .map(|o| {
                let test_solution = i.windows(2).zip(o).map(|(iv, &on)| (on, iv[0], iv[1]));

                // order the solution properly
                let min_outer = test_solution.clone().map(|t| t.0).min().unwrap();
                let min_outer_ind = test_solution
                    .clone()
                    .find_position(|&n| n.0 == min_outer)
                    .unwrap()
                    .0;

                test_solution
                    .cycle()
                    .skip(min_outer_ind)
                    .take(5)
                    .collect_vec()
            })
            .filter(|ts| ts.iter().map(|(i, j, k)| i + j + k).all_equal())
            .collect_vec()
    });

    // dedupe solution sets
    let deduped_solutions: HashSet<Vec<(u32, u32, u32)>> = HashSet::from_iter(solution_sets);

    deduped_solutions
        .iter()
        .map(|tv| {
            tv.iter()
                .map(|t| format!("{}{}{}", t.0, t.1, t.2))
                .collect_vec()
        })
        .map(|sv| {
            sv.iter()
                .map(|s| s.as_ref())
                .collect::<Vec<&str>>()
                .concat()
        })
        .filter(|s| s.len() == 16)
        .map(|s| s.parse::<u64>().unwrap())
        .max()
        .unwrap()
}

fn main() {
    println!("{}", magic_five_gon_ring_max_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(magic_five_gon_ring_max_string(), 6531031914842725);
    }
}
