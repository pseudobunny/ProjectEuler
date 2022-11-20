use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let digits = vec![1,2,3,4,5,6,7,8,9,10];

    let inner = digits.iter()
        .permutations(5)
        .unique();

    let outer: Vec<Vec<Vec<u32>>> = inner.clone()
        .map(|i| digits.iter().filter(|n| !i.contains(n)).map(|&n| n as u32).collect::<Vec<u32>>()) 
        .map(|nv| nv.iter().copied().permutations(5).unique().collect_vec())
        .collect();

    let possible_solution_sets = inner.clone()
        .map(|iv| iv.iter().cycle().take(6).copied().collect_vec())
        .zip(outer);

    let mut solution_sets: Vec<Vec<(u32,u32,u32)>> = vec![];
    for iov in possible_solution_sets.clone() {
        let i = iov.0;
        let ov = iov.1;

        for o in ov { 
            let test_solution = i.windows(2)
                .zip(o)
                .map(|(iv, on)| (on, *iv[0], *iv[1]));

            // order the solution properly
            let min_outer = test_solution.clone().map(|t| t.0).min().unwrap();
            let min_outer_ind = test_solution.clone().find_position(|n| n.0 == min_outer).unwrap().0;
            let test_solution = test_solution.cycle().skip(min_outer_ind).take(5);

            let sums = test_solution.clone().map(|(i,j,k)| i+j+k);
            
            if sums.clone().all_equal() {
                solution_sets.push(test_solution.collect_vec());
            }
        }
    }

    // dedupe solution sets
    let mut deduped_solutions = HashSet::new();
    for solution in solution_sets.clone() {
        deduped_solutions.insert(solution);
    }

    let solution_nums: Vec<u64> = deduped_solutions
        .iter()
        .map(|tv| tv.iter().map(|t| format!("{}{}{}", t.0, t.1, t.2)).collect_vec())
        .map(|sv| sv.iter().map(|s| s.as_ref()).collect::<Vec<&str>>().concat())
        .filter(|s| s.len() == 16)
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    println!("{}", solution_nums.iter().max().unwrap())
}
