
pub fn solve() {
    let limit = 1_000_000;

    let k = (1..limit)
        .map(|n| (n, chain_len(n)))
        .max_by(|x, y| x.1.cmp(&y.1))
        .unwrap();

    println!("Solution to problem 14:");
    println!("{}", k.0);
}

fn next(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn chain_len(n: u64) -> u64 {
    let mut l = 1;
    let mut _n = n;

    while _n != 1 {
        l += 1;
        _n = next(_n);
    }

    // println!("n: {} / len: {}", n, l);

    l
}
