use custom_math_utilities::triangle_number_list;
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

fn word_line_to_names(line: &str) -> Vec<String> {
    line.split(',')
        .map(|s| s.trim_matches('"').to_string())
        .collect()
}

fn convert_word_to_sum(word: &str) -> u8 {
    word.chars()
        .into_iter()
        .map(|c| c as u8)
        .map(|c| (c - 64u8))
        .sum()
}

fn triangle_names_in_file_count(filename: &str) -> usize {
    let input_lines = lines_from_file(filename);

    let mut tri_list = triangle_number_list();

    word_line_to_names(&input_lines[0])
        .into_iter()
        .map(|w| convert_word_to_sum(&w) as u64) // convert to sum
        .filter(|s| tri_list.is_in(*s)) // filter out words that aren't triangle numbers
        .count()
}

fn main() {
    println!("{}", triangle_names_in_file_count("p042_words.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(triangle_names_in_file_count("p042_words.txt"), 162);
    }
}
