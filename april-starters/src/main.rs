use std::{cmp, io};
fn main() {
    let mut testcase_string = String::new();
    io::stdin().read_line(&mut testcase_string).unwrap();
    let testcase: u32 = testcase_string.trim().parse::<u32>().unwrap();
    let mut answers: Vec<&str> = Vec::new();
    for _ in 0..testcase {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let inputs: Vec<u32> = line
            .trim_end_matches('\n')
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let (x, y, xr, yr, d) = (inputs[0], inputs[1], inputs[2], inputs[3], inputs[4]);
        let temp1: u32;
        let temp2: u32;
        temp1 = x / xr;
        temp2 = y / yr;
        if cmp::min(temp1, temp2) >= d {
            answers.push("YES");
        } else {
            answers.push("NO");
        }
    }
    for i in 0..answers.len() {
        println!("{}", answers[i]);
    }
}
