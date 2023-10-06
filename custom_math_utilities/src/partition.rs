// TODO: Make a more generic version of this that is used for both this and generated_func_list
fn pentagonal_number(k: i64) -> i64 {
    (k * (3 * k - 1)) / 2
}

fn generate_k(index: u32) -> i64 {
    ((index + 1) / 2) as i64 * (-1_i64).pow(index - 1)
}

pub fn partition(target: i64) -> i64 {
    let mut partitions = vec![1];

    for n in 1..=target {
        partitions.push(0);

        for i in 1.. {
            let k = generate_k(i);
            let pentagonal = pentagonal_number(k);

            // ensures we stay within usize bounds
            if pentagonal > n {
                break;
            }

            partitions[n as usize] +=
                (-1_i64).pow((k + 1).abs() as u32) * partitions[(n - pentagonal) as usize]
        }
    }

    partitions[target as usize]
}
