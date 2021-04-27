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
    let (a, b): (u32, u32) = (inputs[0], inputs[1]);
    let answer: Vec<u32> = (a..b + 1).into_iter().filter(|&x| x % 2 == 1).collect();
    for i in 0..answer.len() {
        print!("{} ", answer[i]);
    }
}
