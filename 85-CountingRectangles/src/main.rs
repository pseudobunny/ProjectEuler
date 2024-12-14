use memoize::memoize;
use std::cmp::Ordering;

#[memoize]
fn rectangles_in(n: usize, m: usize) -> usize {
    ((n * n + n) * (m * m + m)) / 4
}

fn grid_area_closest_to_rectangles(total: usize) -> usize {
    let cutoff = total * total;
    let mut closest_area = 1;
    let mut closest_distance = total - 1;

    for n in 2.. {
        for m in 1..=n {
            let rect = rectangles_in(n, m);

            if rect > cutoff {
                return closest_area;
            }

            let distance = match total.cmp(&rect) {
                Ordering::Greater => total - rect,
                Ordering::Equal => 0,
                Ordering::Less => rect - total,
            };
            if closest_distance > distance {
                closest_distance = distance;
                closest_area = n * m;
            }
        }
    }

    0
}

fn main() {
    println!("{}", grid_area_closest_to_rectangles(2_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(rectangles_in(1, 1), 1);
        assert_eq!(rectangles_in(2, 1), 3);
        assert_eq!(rectangles_in(2, 2), 9);
        assert_eq!(rectangles_in(3, 1), 6);
        assert_eq!(rectangles_in(3, 2), 18);
        assert_eq!(rectangles_in(3, 3), 36);
    }

    #[test]
    fn q_case() {
        assert_eq!(grid_area_closest_to_rectangles(2_000_000), 2772);
    }
}
