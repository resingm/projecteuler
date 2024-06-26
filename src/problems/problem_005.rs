
pub fn solve() {
    let divisors = (1..21).collect();
    // .into_iter().collect();
    let n = find_number(divisors);

    println!("Solution to problem 5:");
    println!("{:?}", n);
}

fn find_number(divisors: Vec<i64>) -> i64 {
    let start = divisors.iter().max().unwrap() + 1;

    for i in start.. {
        if divisors.iter().all(|d| i % d == 0) {
            return i;
        }
    };

    return -1;
}
