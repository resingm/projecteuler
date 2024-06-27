
pub fn solve() {
    let limit: i64 = 101;

    let sum_of_squares: i64 = (1..limit).map(|e| {
        e * e
    }).into_iter().sum();

    let square_of_sums: i64 = (1..limit).into_iter().sum::<i64>().pow(2);
    let solution = square_of_sums - sum_of_squares;

    println!("Solution to problem 6:");
    println!("{}", solution);
}