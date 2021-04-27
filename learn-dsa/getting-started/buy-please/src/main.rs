use std::io;
fn answer(a: u32, b: u32, x: u32, y: u32) -> u32 {
    a * x + b * y
}
fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    if line.ends_with('\n') {
        line = line[0..line.len() - 1].to_string();
    }
    let inputs: Vec<u32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (a, b, x, y) = (inputs[0], inputs[1], inputs[2], inputs[3]);
    println!("{}", answer(a, b, x, y));
}
