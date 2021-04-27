use std::io;
fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    if line.ends_with('\n') {
        line = line[0..line.len() - 1].to_string();
    }
    let n: u32 = line.parse().unwrap();
    print!(
        "{} {}",
        (1..n * 2 + 1)
            .into_iter()
            .filter(|&x| x % 2 == 1)
            .fold(0, |a, b| a + b),
        (1..n * 2 + 1)
            .into_iter()
            .filter(|&x| x % 2 == 0)
            .fold(0, |a, b| a + b)
    )
}
