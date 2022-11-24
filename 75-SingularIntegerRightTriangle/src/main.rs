use std::collections::HashSet;

fn generate_pythagorean_triplets(n: u64) -> Vec<(u64, u64, u64)> {
    let mut triplets = vec![];

    let mut triplet = (0,0,0);
    let mut m = 2;

    while triplet.2 <= n {
        for n in 1..m {
            triplet.0 = m*m - n*n;
            triplet.1 = 2*m*n;
            triplet.2 = m*m + n*n;

            triplets.push(triplet);
        }
        m += 1;
    }

    triplets
}

fn trip_sum(trip: (u64, u64, u64)) -> u64 {
    trip.0+trip.1+trip.2
}

fn sort_trip(trip: (u64, u64, u64)) -> (u64, u64, u64) {
    let mut t_v = vec![trip.0, trip.1, trip.2];
    t_v.sort();

    (t_v[0], t_v[1], t_v[2])
}

fn main() {
    // Generate all primitive triplets to which c is less than or equal to 1_500_000
    let prim_triplets = generate_pythagorean_triplets(1_500_000);
    
    let mut triplets = HashSet::new();
    for trip in prim_triplets.iter() {
        let mut i = 1;
        loop {
            let new_trip = sort_trip((trip.0 * i, trip.1 * i, trip.2 * i));
            
            if trip_sum(new_trip) > 1_500_000 {
                break;
            }

            triplets.insert(new_trip);

            i += 1;
        }
    }
    
    let trip_sums = triplets.iter().map(|t| trip_sum(*t)).collect::<Vec<u64>>();

    let mut single_sums_set = HashSet::new();
    let mut mul_sums_set = HashSet::new();
    for t_s in trip_sums.iter() {
        if mul_sums_set.contains(t_s) {
            continue;
        }
        if single_sums_set.contains(t_s) {
            single_sums_set.remove(t_s);
            mul_sums_set.insert(t_s);
            continue;
        }

        single_sums_set.insert(t_s);
    }

    println!("{}", single_sums_set.len());
}
