
pub fn solve() {
    let limit: usize = 10001;

    let p: i64 = (2..).filter(|e| is_prime(*e)).take(limit).last().unwrap();

    println!("Solution to problem 7:");
    println!("{:?}", p);
}

fn is_prime(n: i64) -> bool {
    let limit = (n as f64).sqrt() as i64;

    for i in 2..=limit {
        if n % i == 0 {
            return false
        }
    }

    true
}