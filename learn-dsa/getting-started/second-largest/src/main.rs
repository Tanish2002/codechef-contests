use std::io;
fn main() {
    let mut inputs: Vec<u32> = Vec::new();
    for _ in 0..3 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.ends_with('\n') {
            line = line[0..line.len() - 1].to_string();
        }
        inputs.push(line.parse::<u32>().unwrap());
    }
    inputs.sort();
    let answer = inputs[inputs.len() - 2];
    println!("{}", answer);
}
