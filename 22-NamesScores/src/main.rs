use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Something went wrong reading the file");

    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line").trim().to_string())
        .collect()
}

fn name_line_to_names(line: &String) -> Vec<String> {
    line.split(",")
        .map(|s| s.trim_matches('"').to_string())
        .collect()
}

fn score_from_name(name: &String) -> u64 {
    name.to_lowercase()
        .chars()
        .map(|c| c as u64 - 'a' as u64 + 1)
        .sum()
}

fn total_name_scores_from_file(filename: &str) -> u64 {
    let mut names = name_line_to_names(&lines_from_file(filename)[0]);
    names.sort_by_key(|name| name.to_lowercase());

    names
        .iter()
        .enumerate()
        .map(|(i, n)| (i as u64 + 1) * score_from_name(n))
        .sum()
}

fn main() {
    println!("{}", total_name_scores_from_file("names.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(total_name_scores_from_file("names.txt"), 871198282)
    }
}
