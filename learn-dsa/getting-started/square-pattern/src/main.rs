use std::io;
fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    if line.ends_with('\n') {
        line = line[0..line.len() - 1].to_string();
    }
    let input: u32 = line.parse().unwrap();
    for i in 1..input + 1 {
        if i % 2 != 0 {
            println!(
                "{} {} {} {} {}",
                ((i * 5) - 4),
                ((i * 5) - 3),
                (i * 5) - 2,
                (i * 5) - 1,
                (i * 5)
            );
        }
        if i % 2 == 0 {
            println!(
                "{} {} {} {} {}",
                (i * 5),
                ((i * 5) - 1),
                ((i * 5) - 2),
                ((i * 5) - 3),
                ((i * 5) - 4)
            );
        }
    }
}
