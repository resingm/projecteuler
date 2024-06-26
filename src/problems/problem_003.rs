
pub fn solve() {
    // let p: i64 = 13195;
    let p: i64 = 600_851_475_143;
    // let p: i64 = 600_851_475;

    let prime_factors = get_prime_factors(p);
    let solution = prime_factors.last().unwrap();

    println!("Solution to problem 3:");
    println!("{}", solution);
}

fn get_prime_factors(p: i64) -> Vec<i64> {
    let limit = (p as f64).sqrt() as i64;
    let factors: Vec<i64> = (2..limit).into_iter().map(|i| {
        match p % i {
            0 => Some(i),
            _ => None,
        }
    })
        .filter(|e| e.is_some_and(|e| is_prime(e)))
        .map(|e| e.unwrap())
        .collect();

    factors
}


fn is_prime(p: i64) -> bool {
    let limit = (p as f64).sqrt() as i64;

    for i in 2..limit {
        if p % i == 0 {
            return false;
        }
    }

    true
}