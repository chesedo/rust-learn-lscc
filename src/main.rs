use converter_test::convert;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer).unwrap();

    let json = convert(&buffer);

    println!("{json}");
}
