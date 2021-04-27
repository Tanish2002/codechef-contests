use std::io;
fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    if line.ends_with('\n') {
        line = line[0..line.len() - 1].to_string();
    }
    let input: u32 = line.parse().unwrap();
    match (input % 5, input % 6) {
        (0, 0) => println!("YES"),
        (_, 0) => println!("YES"),
        (0, _) => println!("YES"),
        _ => println!("NO"),
    }
}
