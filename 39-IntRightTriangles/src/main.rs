use custom_math_utilities::check_if_whole;

fn solution_num_for_perimeter(n: u64) -> usize {
    let f_n = n as f64;

    (1..(n/3))
        .map(|x| x as f64)
        .map(|x| (f_n*0.5 - x) / (1.0 - (x / f_n))) // calculate the y required to satisfy this equation
        .filter(|&y| check_if_whole(y))
        .count()
}

fn perimeter_with_max_solutions_under(p_max: u64) -> u64 {
    (15..=p_max)
        .map(|n| (n, solution_num_for_perimeter(n)))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0
}

fn main() {
    println!("{}", perimeter_with_max_solutions_under(1000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(perimeter_with_max_solutions_under(1000), 840);
    }
}