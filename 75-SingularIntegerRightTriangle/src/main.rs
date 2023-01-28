use std::collections::{HashMap, HashSet};

fn generate_pythagorean_triplets(n: u64) -> Vec<(u64, u64, u64)> {
    let mut triplets = vec![];

    let mut triplet = (0, 0, 0);
    let mut m = 2;

    while triplet.2 <= n {
        for n in 1..m {
            triplet.0 = m * m - n * n;
            triplet.1 = 2 * m * n;
            triplet.2 = m * m + n * n;

            triplets.push(triplet);
        }
        m += 1;
    }

    triplets
}

fn trip_sum(trip: (u64, u64, u64)) -> u64 {
    trip.0 + trip.1 + trip.2
}

fn sort_trip(trip: (u64, u64, u64)) -> (u64, u64, u64) {
    let mut t_v = vec![trip.0, trip.1, trip.2];
    t_v.sort();

    (t_v[0], t_v[1], t_v[2])
}

fn num_single_trip_sets_under(n: u64) -> usize {
    // Generate all primitive triplets to which c is less than or equal to 1_500_000
    let prim_triplets = generate_pythagorean_triplets(n);

    let mut triplets = HashSet::new();
    for trip in prim_triplets.iter() {
        for i in 1.. {
            let new_trip = sort_trip((trip.0 * i, trip.1 * i, trip.2 * i));

            if trip_sum(new_trip) > 1_500_000 {
                break;
            }

            triplets.insert(new_trip);
        }
    }

    let trip_sums = triplets.iter().map(|t| trip_sum(*t));

    let mut trip_sum_by_count = HashMap::<u64, usize>::new();
    for t_s in trip_sums {
        trip_sum_by_count
            .entry(t_s)
            .and_modify(|s| *s += 1)
            .or_insert(1);
    }

    trip_sum_by_count.into_values().filter(|&c| c == 1).count()
}

fn main() {
    println!("{}", num_single_trip_sets_under(1_500_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(num_single_trip_sets_under(1_500_000), 161667);
    }
}
