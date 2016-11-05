use std::io;
use std::cmp::Ordering;

fn main() {
    let mut line1 = String::new();
    let mut line2 = String::new();

    io::stdin().read_line(&mut line1).ok().expect("Read error");
    io::stdin().read_line(&mut line2).ok().expect("Read error");

    let a: Vec<i32> = line_to_vector(&mut line1);
    let b: Vec<i32> = line_to_vector(&mut line2);
    let (mut c, mut d) = (0, 0);

    for (i, _v) in (0..a.len()).enumerate() {
        match a[i].cmp(&b[i]) {
            Ordering::Less => d = d + 1,
            Ordering::Greater => c = c + 1,
            Ordering::Equal => continue
        }
    }

    println!("{} {}", c, d);
}

// convert a line of space-separated integers into a vector of integers
fn line_to_vector(line: &mut String) -> Vec<i32> {
    line.trim().split(" ")
        .map(|x| x.parse::<i32>().expect("err"))
        .collect::<Vec<_>>()
}
