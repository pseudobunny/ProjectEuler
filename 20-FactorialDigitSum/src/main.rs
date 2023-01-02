fn main() {
    // We don't need to calculate the factorial ourselves here - this is just a sum of the digits in the number
    // If we do end up calculating the result of the factorial, the program would be essentially the same... just
    // with a function result right here.
    let factorial_result = "93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000";

    println!(
        "{}",
        factorial_result
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum::<u32>()
    )
}
