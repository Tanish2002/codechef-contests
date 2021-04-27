use std::io;
fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    if line.ends_with('\n') {
        line = line[0..line.len() - 1].to_string();
    }
    let input: u32 = line.parse().unwrap();
    for i in 1..input + 1 {
        let mut j: u32 = 0;
        while j < input - i {
            print!(" ");
            j += 1
        }
        let mut k: u32 = 0;
        while k < i {
            print!("*");
            k += 1;
        }
        println!();
    }
}
