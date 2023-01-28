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

fn line_to_vec(line: &str) -> Vec<u64> {
    line.trim()
        .split(',')
        .map(|ns| ns.parse::<u64>().expect("Could not parse num"))
        .collect::<Vec<u64>>()
}

fn lines_to_mat(lines: Vec<String>) -> Vec<Vec<u64>> {
    lines
        .iter()
        .map(|l| line_to_vec(l))
        .collect::<Vec<Vec<u64>>>()
}

fn min_path_sum(filename: &str) -> u64 {
    let file_lines = lines_from_file(filename);
    let mut mat = lines_to_mat(file_lines);

    let h = mat.len();
    let w = mat[0].len();

    // handle first column
    for i in 1..h {
        mat[i][0] += mat[i - 1][0];
    }

    // handle first row
    for j in 1..w {
        mat[0][j] += mat[0][j - 1];
    }

    for i in 1..h {
        for j in 1..w {
            mat[i][j] += std::cmp::min(mat[i - 1][j], mat[i][j - 1]);
        }
    }

    mat[h - 1][w - 1]
}

fn main() {
    println!("{:?}", min_path_sum("src/p081_matrix.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(min_path_sum("src/p081_matrix.txt"), 427337);
    }
}
