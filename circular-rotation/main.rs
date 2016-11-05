use std::io;

fn main() {
    let mut line1 = String::new();
    let mut line2 = String::new();
    io::stdin().read_line(&mut line1).expect("Read error");
    io::stdin().read_line(&mut line2).expect("Read error");

    // extract n, k, q from line 1
    let v = line_to_vector(&mut line1, " ");
    let (_n, k, q) = (v[0], v[1], v[2]);

    // read line2 into a vector
    let a = line_to_vector(&mut line2, " ");

    // read remaining q-lines into a vector
    let mut line = String::new();
    for _i in 0..q {
        io::stdin().read_line(&mut line).expect("Read error");
    }
    let queries = line_to_vector(&mut line, "\n");

    // rotate the input
    let res = rotate(a, k as usize);

    // print each of the queries
    for m in queries {
        println!("{}", res[m as usize])
    }
}

// convert delimited strings to vector of i32
fn line_to_vector(line: &mut String, delim: &str) -> Vec<i32> {
    line.trim().split(delim)
        .map(|x| x.parse::<i32>().expect("Parse error"))
        .collect::<Vec<i32>>()
}

// rotate vector k times
fn rotate(a: Vec<i32>, k: usize) -> Vec<i32> {
    let len = a.len();
    let mut out = vec![0i32; len];
    for i in 0..len {
        let n: usize = (i+k)%len;
        out[n] = a[i];
    }
    out
}
