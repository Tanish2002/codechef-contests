use std::io;
fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    if line.ends_with('\n') {
        line = line[0..line.len() - 1].to_string();
    }
    let input: u32 = line.parse().unwrap();
    match (input % 5, input % 11) {
        (0, 0) => println!("BOTH"),
        (0, _) => println!("ONE"),
        (_, 0) => println!("ONE"),
        _ => println!("NONE"),
    }
}
