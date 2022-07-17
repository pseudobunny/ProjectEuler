use std::{
    fs::File,
    io::{prelude::*, BufReader},
    cmp,
};

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename)
        .expect("Something went wrong reading the file");
    
    let buf = BufReader::new(file);
    
    buf.lines()
       .map(|l| l.expect("Could not parse line"))
       .collect()
}

fn line_to_vec(line: &str) -> Vec<u64> {
    let mut line_nums: Vec<u64> = vec![];

    for num_str in line.trim().split(" ") {
        line_nums.push(num_str.parse::<u64>().expect("Could not parse num"));
    }

    line_nums
}

fn condense_line(line: Vec<u64>) -> Vec<u64> {
    let mut condensed_line: Vec<u64> = vec![];

    for n in 0..line.len()-1 {
        condensed_line.push(cmp::max(line[n],line[n+1]));
    }

    condensed_line
}

fn merge_vectors(line1: Vec<u64>, line2: Vec<u64>) -> Vec<u64> {
    let mut merged_vectors: Vec<u64> = vec![];

    for n in 0..line1.len() {
        merged_vectors.push(line1[n]+line2[n]);
    }

    merged_vectors
}

fn find_maximum_path_sum(filename: &str) -> u64 {
    let lines = lines_from_file(filename);
    
    let mut lines_as_nums: Vec<Vec<u64>> = vec![];
    for line in lines {
        lines_as_nums.push(line_to_vec(&line));
    }

    let mut prev_line: Vec<u64> = condense_line(lines_as_nums.pop().expect(""));
    let rev_lines_as_nums: Vec<Vec<u64>> = lines_as_nums.into_iter().rev().collect();
    for line in rev_lines_as_nums {
        prev_line = merge_vectors(prev_line,line);

        if prev_line.len() > 1 {
            prev_line = condense_line(prev_line);
        }
    }

    prev_line[0]
}

fn main() {
    println!("Answer to Problem 18: {}", find_maximum_path_sum("treePathP18.txt"));
    println!("Answer to Problem 67: {}", find_maximum_path_sum("treePathP67.txt"));
}