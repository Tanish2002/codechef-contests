use std::io;
fn triangle_type(a: u32, b: u32, c: u32) -> i8 {
    if a == b && b == c {
        1
    } else if a == b || b == c || c == a {
        2
    } else {
        3
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
    if a + b <= c || a + c <= b || b + c <= a {
        print!("-1");
    } else {
        print!("{}", triangle_type(a, b, c));
    }
}
