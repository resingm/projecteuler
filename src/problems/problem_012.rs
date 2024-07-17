use std::collections::HashSet;

pub fn solve() {
    // let limit = 5;
    let limit: usize = 500;

    let mut t = 0;
    let mut max = 1;

    let mut primes: Vec<u64> = vec![];

    for i in 1.. {
        t = t + &i;

        // Add new primes to prime vector
        if is_prime(&i) {
            primes.push(i);
        }

        // Determine the set of divisors
        /*
        let mut divisors: HashSet<u64> = HashSet::new();
        let n = (t as f64).sqrt() as u64;
        let prime_divisors = primes.iter()
            .filter(|e| e < n)
            .filter(|e| t % *e == 0)

        // for e in primes.iter().filter(|e| t % *e == 0) {
        for e in prime_divisors {
            divisors.insert(e.clone());
            divisors.insert(t / e);
        }

        if divisors.len() > max {
            println!("i: {} / t: {} / d: {}", i, t, divisors.len());
            max = divisors.len();
        }
        */

        let d = number_of_divisors(t);

        if d > max {
            println!("i: {} / t: {} / d: {}", i, t, d);
            max = d;
        }

        if d >= limit {
            println!("Solution to problem 12:");
            println!("{}", t);
            break;
        }
    }
}


// TODO: Do a recursive call instead:
/*
 * def divs(n, m):
 *     if m == 1: return [1]
 *     if n % m == 0: return [m] + divs(n, m - 1)
 *     return divs(n, m - 1)
*/


// fn nth_triangle_number(i: u64) -> u64 {
//     (1..=i).sum()
// }

fn number_of_divisors(i: u64) -> usize {
    let n = (i as f64).sqrt() as u64 + 1;
    let mut divisors: HashSet<u64> = HashSet::new();

    for e in (1..=n).filter(|e| i % e == 0).into_iter() {
        divisors.insert(e);
        divisors.insert(i / e);
    }

    divisors.len()
}

fn is_prime(n: &u64) -> bool {
    let p = (n.clone() as f64).sqrt() as u64;

    for i in 2..=p {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}
