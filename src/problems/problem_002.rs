
pub fn solve() {
    let mut sum = 0;

    let mut a: usize = 1;
    let mut b: usize = 1;

    loop {
        let next = a + b;
        a = b;
        b = next;

        if next % 2 == 0 {
            sum += next;
        }

        if next > 4_000_000 {
            break;
        }
    }

    println!("Solution to problem 2:");
    println!("{}", sum);
}