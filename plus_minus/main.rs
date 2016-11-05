use std::io;

fn main() {
    //
    let mut line1 = String::new();
    let mut line2 = String::new();

    io::stdin().read_line(&mut line1).expect("Read error");
    io::stdin().read_line(&mut line2).expect("Read error");

    let n: f32 = line1.trim().parse().expect("Parse error");

    // read data into a vector
    let data = line_to_vector(&mut line2);

    // filter positive elements
    let pos: f32 = data.iter().filter(|&x| *x > 0).collect::<Vec<_>>().len() as f32;
    let neg: f32 = data.iter().filter(|&x| *x < 0).collect::<Vec<_>>().len() as f32;
    let zero: f32 = n-pos-neg;

    // print out fractions
    println!("{}", pos/n);
    println!("{}", neg/n);
    println!("{}", zero/n);
}

// convert a line of space-separated integers into a vector of integers
fn line_to_vector(line: &mut String) -> Vec<i32> {
    line.trim().split(" ")
        .map(|x| x.parse::<i32>().expect("err"))
        .collect::<Vec<i32>>()
}
