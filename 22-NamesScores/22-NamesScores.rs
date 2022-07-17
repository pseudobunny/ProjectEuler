use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename)
        .expect("Something went wrong reading the file");
    
    let buf = BufReader::new(file);
    
    buf.lines()
       .map(|l| l.expect("Could not parse line").trim().to_string())
       .collect()
}

fn name_line_to_names(line: &String) -> Vec<String> {
    line.split(",").map(|s| s.trim_matches('"').to_string()).collect()
}

fn score_from_name(name: &String) -> u64 {
    let mut sum: u64 = 0;

    for c in name.to_lowercase().chars() {
        sum += c as u64 - 'a' as u64 + 1;
    }

    sum
}

fn main() {
    let lines = lines_from_file("names.txt");
    let mut names = name_line_to_names(&lines[0]);
    names.sort_by_key(|name| name.to_lowercase());

    let mut sum: u64 = 0;
    for n in 0..names.len() {
        sum += score_from_name(&names[n])*(n+1) as u64;
    }

    println!("{}", sum)
}