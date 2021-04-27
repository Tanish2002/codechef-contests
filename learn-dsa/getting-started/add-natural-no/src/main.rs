use std::io;
fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    if line.ends_with('\n') {
        line = line[0..line.len() - 1].to_string();
    }
    let input: u128 = line.parse().unwrap();
    print!("{}", (0..input + 1).fold(0, |a, b| a + b));
}
