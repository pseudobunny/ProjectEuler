use custom_math_utilities::check_if_whole;

fn find_triplets(p: f32) -> Vec<(f32, f32, f32)> {
    let x_max = (p / 2.0).round() as usize;

    (1..x_max)
        .map(|x| x as f32)
        .map(|x| (x, (p / 2.0 - x) / (1.0 - (x / p))))
        .filter(|xy| check_if_whole(xy.1))
        .map(|xy| (xy.0, xy.1, (xy.0 * xy.0 + xy.1 * xy.1).sqrt()))
        .collect()
}

fn triplet_sum(t: (f32, f32, f32)) -> f32 {
    t.0 * t.1 * t.2
}

fn main() {
    println!("{}", triplet_sum(find_triplets(1000.0)[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(triplet_sum(find_triplets(12.0)[0]), 60.0)
    }

    #[test]
    fn q_case() {
        assert_eq!(triplet_sum(find_triplets(1000.0)[0]), 31875000.0)
    }
}
