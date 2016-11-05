use std::io;

fn main() {
    // read input
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Read error");
    let n: u8 = line.trim().parse().expect("Parse error");

    // construct output
    let mut out = String::new();
    for row in 0..n {
        for _col in 0..n-row-1 {
            out = out + " ";
        }
        for _col in 0..row+1 {
            out = out + "#";
        }
        out = out + "\n";
    }

    println!("{}", out);
}
