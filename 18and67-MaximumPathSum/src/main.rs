use std::{
    cmp,
    fs::File,
    io::{prelude::*, BufReader},
};

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Something went wrong reading the file");

    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn condense_line(line: &Vec<u64>) -> Vec<u64> {
    if line.len() > 1 {
        line.windows(2).map(|w| cmp::max(w[0], w[1])).collect()
    } else {
        line.clone()
    }
}

fn merge_vectors(line1: &Vec<u64>, line2: &Vec<u64>) -> Vec<u64> {
    line1
        .iter()
        .zip(line2.iter())
        .map(|ab| ab.0 + ab.1)
        .collect()
}

fn find_maximum_path_sum(filename: &str) -> u64 {
    let lines = lines_from_file(filename);

    let lines_as_nums = lines
        .iter()
        .map(|line| {
            line.trim()
                .split(" ")
                .map(|num_str| num_str.parse::<u64>().expect("Could not parse num"))
                .collect()
        })
        .collect::<Vec<Vec<u64>>>();

    lines_as_nums
        .iter()
        .rev()
        .fold(vec![0; lines_as_nums.last().unwrap().len()], |acc, elem| {
            condense_line(&merge_vectors(&acc, elem))
        })[0]
}

fn main() {
    println!(
        "Answer to Problem 18: {}",
        find_maximum_path_sum("treePathP18.txt")
    );
    println!(
        "Answer to Problem 67: {}",
        find_maximum_path_sum("treePathP67.txt")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p18_case() {
        assert_eq!(find_maximum_path_sum("treePathP18.txt"), 1074)
    }

    #[test]
    fn p67_case() {
        assert_eq!(find_maximum_path_sum("treePathP67.txt"), 7273)
    }
}
