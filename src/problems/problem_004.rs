use std::iter::zip;

pub fn solve() {

    let palindromes = find_palindromes(1000);
    let p = palindromes.iter().max().unwrap();

    println!("Solution to problem 4:");
    println!("{:?}", p);
}

fn find_palindromes(limit: i64) -> Vec<i64> {
    let mut vec: Vec<i64> = Vec::new();

    for i in (1..limit).rev() {
        for j in (1..limit).rev() {
            let candidate: i64 = i * j;
            if is_palindrome(candidate) {
                vec.push(candidate);
            }
        }
    }

    return vec;
}

fn is_palindrome(candidate: i64) -> bool {

    // let s: Vec<char> = candidate.to_string().chars().collect();

    let s = candidate.to_string();
    let center = s.len() / 2;

    let (s1, s2) = s.split_at(center);
    let s1 = String::from(s1);
    let s2 = String::from(reverse_string(s2));

    // let s1: Iter<char> = s[0..center].into_iter();
    // let s2: Iter<char> = s[center..].into_iter().rev();
    let s: Vec<(char, char)> = zip(s1.chars(), s2.chars()).collect();

    s.into_iter().all(|(a, b)| a == b)    
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}
