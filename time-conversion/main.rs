use std::io;

fn main() {
    // get input
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Read error");

    // extract the time portion of the input into a vector of integers vec![h:m:s]
    let mut time: Vec<u8> = line[..8]
        .split(":")
        .map(|x| x.parse::<u8>().expect("Parse error"))
        .collect::<Vec<u8>>();

    // convert 12 --> 0
    time[0] = time[0]%12;

    // if the suffix is PM, add 12 to hour integer
    if &line[8..10] == "PM" {
        time[0] += 12;
    }

    // co-erce the time back into a string
    let output: String = time.iter()
        .map(u8_to_str) // convert to string and pad
        .collect::<Vec<String>>() // collect into vector
        .join(":"); // glue with semicolons

    println!("{}", output)
}

// parse an integer to a padded string
fn u8_to_str(x: &u8) -> String {
    let mut out: String = x.to_string();
    if out.len() == 1 {
        out = "0".to_string()+&out;
    }
    out
}
