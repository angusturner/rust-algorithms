use std::io;

fn main() {
    let mut line1 = String::new();
    let mut line2 = String::new();

    io::stdin().read_line(&mut line1).ok().expect("Read error");
    io::stdin().read_line(&mut line2).ok().expect("Read error");

    let _n: u8 = line1.trim().parse().expect("Parse error");
    let a: Vec<i64> = line_to_vector(&mut line2);
    let result: i64 = a.iter().fold(0i64, |sum, &val| sum+val);
    println!("{}", result);
}

// convert a line of space-separated integers into a vector of integers
fn line_to_vector(line: &mut String) -> Vec<i64> {
    line.trim().split(" ")
        .map(|x| x.parse::<i64>().expect("err"))
        .collect::<Vec<i64>>()
}
