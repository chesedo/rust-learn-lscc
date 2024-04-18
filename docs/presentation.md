---
marp: true
author: Pieter Engelbrecht
theme: uncover
paginate: skip
---

<style>
:root {
  --color-fg-default: #fff;
  font-size: 1.7em;
}
h1, h2, h3 {
    margin-top: 0;
    margin-bottom: 5px;
}
h1 {
    font-size: 1.5em;
}
h2 {
    font-size: 1.2em;
}
h3 {
    font-size: 1em;
}
</style>

# Rust Workshop
## A Rust command line tool to convert XML to JSON

---

<!-- paginate: true -->

## Plan
20 min - Showing fundamentals needed to complete the CLI
40 min - to allow you all to code
20 min - question, show and tell

https://chesedo.github.io/rust-learn-lscc
![bg right 70%](qr.svg)

---

## Cargo

```bash
# To make a boiler plate binary project
$ cargo init

# Format files in an opinionated way
$ cargo fmt

# Basic linting and more advance suggestions
$ cargo check
$ cargo clippy

# Add and remove packages - called crates
$ cargo add
$ cargo remove

# To test or run
$ cargo test
$ cargo run
```

---

## Hello world!

```rust
// For stdout
println!("What is your name?");

// Reading from stdin
let mut name = String::new();
std::io::stdin().read_line(&mut name).unwrap();

// To read the whole input (use std::io::Read;)
// std::io::stdin().read_to_string(&mut name).unwrap();

let name = name.trim();

// Stdout with variables
println!("Hello {name}!");
println!("Your name is {} characters long", name.len());
```

---

## Syntax basics

### Variables
```rust
let x: i32 = 5; // Being explicit about the type
let y = 6;      // Mostly types are inferred

let mut z = 9;  // Defaults to immutable
z += 1;
```

### Logic
```rust
let message = if z > 10 {
    "Greater than 10"
} else {
    "Less than 10"
}; // Closing the expression with a semi-colon is important
```

---

### Loops
```rust
for item in vec![5, 2, 1] {
    println!("item = {item}");
}

for i in 0..5 {
    println!("i = {i}");
}

let mut j = 5;

while j > 0 {
    println!("j = {j}");

    j -= 1;
}
```

---

## Types

```rust
let i: i32 = 5;   // Also has u32 and usize
let f: f32 = 0.43 // Also has f64
let b: bool = true;
let s: String = "text".to_string();
// let s = "text";
let (a, b) = (5, 0.43);
let v = vec![5, 2, 1];
let h = HashMap::new();
```

---

## Types part 2

```rust
enum Going {
    Yes,
    No,
    Maybe(String),
};

let e = Going::Yes;

struct Person {
    first_name: String,
    last_name: String,
};

let p = Person {
    first_name: "Pieter".to_string(),
    last_name: "Engelbrecht".to_string(),
};
```

---

## Functions

```rust
fn concat(part1: String, part2: String) -> String {
    // return format!("{part1}{part2}"); // Equals the below
    format!("{part1} {part2}") // The missing semi-colon is on purpose
}
```

---

## Beginner cheats
<span style="color: #ca880f">These tips are only for those new to Rust!</span>

```rust
concat("hello", "world"); // Error: expected `String`, found `&str`

// Just `.to_string()`
```

```rust
let full_name = concat(p.first_name, p.last_name);

println!("I now know your fullname {}", p.first_name); // Error: borrow / use of moved value

// Just `.clone()` the first uses... or every use :)
```

---

## Beginner cheats part 2

```rust
let age = "32".parse(); // We don't want a `Result`

// Just `.unwrap()`
```

```rust
// Expected reference
// Will see in a bit?
```

```rust
// Derive Debug
println!("{p:?}"); // Error: doesn't implement `Debug`

// Add `#[derive(Debug)]` on top of the struct definition
```
---

## Serde (https://docs.rs/serde)

Is a **Ser**ialization and **De**serialization library
Libraries are called crates in Rust
This one just the generic "parent" crate with the abstractions
So there are implementations for almost every type like:
- JSON
- Proto
- YAML

```
$ cargo add serde --features derive
$ cargo add serde_json
```

---

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}
```

---

```rust
fn main() {
    let point = Point { x: 1, y: 2 };
    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}

```

---

### serde_json (https://docs.rs/serde_json)
Gives us access to a generic `Value` type

```rust
use serde_json::{json, Value};

fn main() {
    let point = json!({ "x": 1, "y": 2 });
    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Value = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}
```

---

### quick-xml (https://docs.rs/quick-xml)
```
$ cargo add quick-xml
```

For serializing and deserializing XML using Serde

---

# Now we build
... using the building blocks covered ...

---

## Skeleton

```rust
fn main() {
    // Read input from stdin

    // Deserialize XML to some struct

    // Serialize the struct to JSON

    // Print to stdout
}
```

---

```rust
use std::io::Read;

fn main() {
    // Read input from stdin
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    // ...
}
```

---

```rust
fn main() {
    // ...

    // Deserialize XML to some struct
    let value: serde_json::Value = quick_xml::de::from_str(&input).unwrap();

    // ...
}
```

---

```rust
fn main() {
    // ...

    // Serialize the struct to JSON
    let output = serde_json::to_string_pretty(&value).unwrap();

    // ...
}
```

---

```rust
fn main() {
    // ...

    // Print to stdout
    println!("{output}");
}
```

---

```rust
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
```

---

## Extras

### Match statement

```rust
match e {
    Going::Yes => println!("Going"),
    Going::No => println!("Not going"),
    Going::Maybe(reason) => println!("Might go because {reason}"),
}
```

---

### Option
Since Rust wants to be a safe language it does not have `null`s. That means the absence of a value is strongly typed and has to be handled explicitly.

```rust
let age = Some(32);

// let age: Option<i32> = None;

match age {
    Some(age) => println!("You are {age} years old"),
    None => println!("Your age is a mystery"),
}

// Almost the same as the above
if let Some(age) = age {
    println!("You are {age} years old");
} else {
    println!("Your age is a mystery");
}
```

---

### Clap

```bash
$ cargo add clap --features derive
```

```rust
use clap::Parser; // Both a macro and a trait in this case

/// Fancy program which greets you by name
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Optional name to be greeted by
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.name {
        Some(name) => println!("Hello, {}!", name),
        None => println!("Hello, world!"),
    }
}
```

---

# Extended build time
Edit the solution to optionally read from a file
... or optionally write to a file

---

```rust
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

    // ...
}
```

---

## Things to try at home
- Modules and functions refactoring
- Playing with function comments / documentation
- Better error handling
  - Using the `Result` type with `String` errors
  - Switching to `anyhow` or `thiserror` crates for errors
- Make function arguments `&str` instead of `String`
  - Read up on the Rust ownership model
- Custom deserialize to get rid of `$text`
  - Look at https://serde.rs