use std::io::Read;

fn main() {
    // Read input from stdin
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    // Deserialize XML to some struct
    let value: serde_json::Value = quick_xml::de::from_str(&input).unwrap();

    // Serialize the struct to JSON
    let output = serde_json::to_string_pretty(&value).unwrap();

    // Print to stdout
    println!("{output}");
}
