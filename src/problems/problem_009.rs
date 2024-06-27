

struct TupleGenerator {
    a: u64,
    b: u64,
    c: u64,
    limit: u64,

}

impl TupleGenerator {
    fn new(limit: u64) -> Self {
        Self { a: 1, b: 1, c: 1, limit }
    }
}

impl Iterator for TupleGenerator {
    type Item = (u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.a, self.b, self.c);

        if self.a > self.limit {
            return None
        }

        if self.c < self.limit {
            self.c += 1;
        } else if self.b < self.limit {
            self.c = 1;
            self.b += 1;
        } else {
            self.c = 1;
            self.b = 1;
            self.a += 1;
        }

        Some(result)
    }
}



pub fn solve() {
    let limit: u64 = 1000;

    let generator = TupleGenerator::new(limit);

    let (a, b, c) = generator
        .filter(|&(a, b, c)| is_sum_eq(a, b, c, limit))
        .filter(|&(a, b, c)| is_pythagorean_triplet(a, b, c))
        .next().unwrap();

    let p = a * b * c;
    
    println!("Solution to problem 9:");
    println!("{:?}", p);
}

fn is_sum_eq(a: u64, b: u64, c: u64, eq: u64) -> bool {
    a + b + c == eq
}

fn is_pythagorean_triplet(a: u64, b: u64, c: u64) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2)
}
