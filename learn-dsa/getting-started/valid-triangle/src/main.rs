use std::io;
fn valid_triangle(a: u32, b: u32, c: u32) -> bool {
    if a + b <= c || a + c <= b || b + c <= a {
        false
    } else {
        true
    }
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
    let (a, b, c) = (inputs[0], inputs[1], inputs[2]);
    match valid_triangle(a, b, c) {
        true => println!("YES"),
        false => println!("NO"),
    }
}
