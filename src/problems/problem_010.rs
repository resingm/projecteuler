pub fn solve() {
    let limit: u64 = 2_000_000;

    let sum: u64 = (2..limit).filter(|x| is_prime(x)).sum();

    println!("Solution to problem 10:");
    println!("{:?}", sum);
}


fn is_prime(n: &u64) -> bool {
    let n = n.clone();
    let limit = (n as f64).sqrt() as u64;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }

    true
}