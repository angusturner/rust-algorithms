use std::io;

fn main() {
    // collect first line, n
    let mut line1 = String::new();
    io::stdin().read_line(&mut line1).expect("Read error");
    let n: usize = line1.trim().parse().expect("Parse error");

    // collect remaining lines into a 2D vector
    let mut data = vec![];
    let mut line;
    for _i in 0..n {
        line = String::new();
        io::stdin().read_line(&mut line).expect("Read error");
        data.push(line_to_vector(&mut line));
    }

    // extract the two diagonals
    let mut diagonal1: Vec<i32> = vec![];
    let mut diagonal2: Vec<i32> = vec![];
    for row in 0..n {
        for col in 0..n {
            if row == col {
                diagonal1.push(data[row][col]);
            }
            if row == n-1-col {
                diagonal2.push(data[row][col]);
            }
        }
    }

    // compute the sum
    let sum1: i32 = diagonal1.iter().fold(0, |acc, &val| acc+val);
    let sum2: i32 = diagonal2.iter().fold(0, |acc, &val| acc+val);

    // print absolute difference
    println!("{}", (sum1-sum2).abs());
}

fn line_to_vector(line: &mut String) -> Vec<i32> {
    line.trim().split(" ")
        .map(|x| x.parse::<i32>().expect("Error parsing number"))
        .collect::<Vec<i32>>()
}
