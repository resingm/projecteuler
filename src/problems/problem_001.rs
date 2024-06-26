
pub fn solve() {

    let mut sum = 0;

    for i in 1..1000 {
        //match i {
        //    i % 3 == 0 => sum += i,
        //    i % 5 == 0 => sum += i,
        //};

        if i % 3 == 0 {
            sum += i;
        } else if i % 5 == 0 {
            sum += i;
        }
    }

    println!("Solution to problem 1:");
    println!("{}", sum);
}
