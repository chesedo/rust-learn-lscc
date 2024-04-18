use std::{fs, io::Read};

use clap::Parser;

/// Convert XML to JSON
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Optional file to read XML from
    #[arg(short, long)]
    input: Option<String>,
}

fn main() {
    let args = Args::parse();

    let xml = match args.input {
        Some(file) => fs::read_to_string(file).expect("To be able to read the file"),
        None => {
            // Read input from stdin
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input).unwrap();
            input
        }
    };

    // Deserialize XML to some struct
    let value: serde_json::Value = quick_xml::de::from_str(&xml).unwrap();

    // Serialize the struct to JSON
    let output = serde_json::to_string_pretty(&value).unwrap();

    // Print to stdout
    println!("{output}");
}
