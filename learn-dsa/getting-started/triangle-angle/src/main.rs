use std::io;
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
    if a + b + c == 180 && a != 0 && b != 0 && c != 0 {
        print!("YES")
    } else {
        print!("NO")
    }
}
