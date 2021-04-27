use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("failed to read line");
    let num: u64 = s.trim().parse().expect("Enter in a positive integer only");
    let factors = get_factors_functional(num);
    println!("{}", factors.len());
    for i in 0..factors.len() {
        print!("{} ", factors[i]);
    }
}

fn get_factors_functional(n: u64) -> Vec<u64> {
    (1..n + 1)
        .into_iter()
        .filter(|&x| n % x == 0)
        .collect::<Vec<u64>>()
}
