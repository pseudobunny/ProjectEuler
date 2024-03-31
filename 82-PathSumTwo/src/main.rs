use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

use priority_queue::DoublePriorityQueue;

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct Point {
    row: usize,
    col: usize,
}

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

fn lines_to_mat(lines: Vec<String>) -> (Vec<Vec<u64>>, usize, usize) {
    let num_lines = lines
        .iter()
        .map(|l| line_to_vec(l))
        .collect::<Vec<Vec<u64>>>();

    let height = num_lines.len();
    let width = num_lines[0].len();

    (num_lines, height, width)
}

fn shortest_path(filename: &str) -> u64 {
    let file_lines = lines_from_file(filename);
    let (mat, rows, cols) = lines_to_mat(file_lines);

    let mut curr_min = u64::MAX;

    for start in 0..rows {
        let mut queue = DoublePriorityQueue::new();

        let points = (0..rows).flat_map(|row| (0..cols).map(move |col| Point { row, col }));
        for point in points {
            queue.push(point, u64::MAX);
        }

        queue.change_priority(&Point { row: start, col: 0 }, mat[start][0]);

        while !queue.is_empty() {
            // because this loop continues while the queue is not empty, this is guaranteed
            let (curr_point, curr_dist) = queue.pop_min().unwrap();

            if curr_point.col + 1 == cols {
                curr_min = std::cmp::min(curr_min, curr_dist);
                continue;
            }

            if curr_point.row != 0 {
                if let Some((&neighbor_point, &neighbor_dist)) = queue.get(&Point {
                    row: curr_point.row - 1,
                    col: curr_point.col,
                }) {
                    let new_dist = std::cmp::min(
                        neighbor_dist,
                        curr_dist + mat[neighbor_point.row][neighbor_point.col],
                    );
                    queue.push_decrease(neighbor_point, new_dist);
                }
            }

            if let Some((&neighbor_point, &neighbor_dist)) = queue.get(&Point {
                row: curr_point.row,
                col: curr_point.col + 1,
            }) {
                let new_dist = std::cmp::min(
                    neighbor_dist,
                    curr_dist + mat[neighbor_point.row][neighbor_point.col],
                );
                queue.push_decrease(neighbor_point, new_dist);
            }

            if curr_point.row + 1 != rows {
                if let Some((&neighbor_point, &neighbor_dist)) = queue.get(&Point {
                    row: curr_point.row + 1,
                    col: curr_point.col,
                }) {
                    let new_dist = std::cmp::min(
                        neighbor_dist,
                        curr_dist + mat[neighbor_point.row][neighbor_point.col],
                    );
                    queue.push_decrease(neighbor_point, new_dist);
                }
            }
        }
    }

    curr_min
}

fn main() {
    println!("{}", shortest_path("src/0082_matrix.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(shortest_path("src/test_matrix.txt"), 994);
    }
}
