---
marp: true
theme: default
paginate: true
---

# Rust Workshop
## A Rust command line tool to convert XML to JSON

---

## Plan
20 min - Showing fundamentals needed to complete the CLI
40 min - to allow you all to code
20 min - question, show and tell

LINK / QR code

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
let mut stdin = io::stdin();
stdin.read_to_string(&mut name).unwrap();

// With variables
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
let can_vote = if has_id && has_photo {
    true
} else {
    false
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

### Loops part 2

```rust
loop {
    if some_condition {
        break;
    }

    if something_special {
        continue;
    }

    // "special" cases won't reach here
}
```

---

## Types

```rust
let i: i32 = 5;   // Also has u32 and usize
let x: f32 = 0.43 // Also has f64
let b: bool = true;
let s: String = "text".to_string();
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
    Maybe,
};

let e = Going::Yes;

// Maybe match?

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
> These tips are only for those new to Rust!

```rust
concat("hello", "world"); // Error: expected String, got &str

// Just `.to_string()`
```

```rust
let full_name = concat(p.first_name, p.last_name);

println!("I now know your fullname {}", p.first_name); // Use of moved value

// Just `.clone()` the first uses... or every use :)
```

```rust
// Unused `Result`

// Just `.unwrap()`
```

---

## Documentation and Testing
```rust
/// Joins two strings with a space
/// 
/// # Example
/// ```
/// let actual = concat("Hi".to_string(), "there".to_string());
/// let expected = "hi there".to_string();
/// 
/// assert_eq!(actual, expected, "optional message");
/// ```
fn concat(part1: String, part2: String) -> String {
    format!("{part1} {part2}")
}
```

---

## Testing part 2
```rust
#[cfg(test)]
mod tests {}
```

> I won't cover modules so can't go here

---

## Serde (https://doc.rs/serde)

Is **Ser**ialization and **De**serialization library
Libraries are called crates in Rust
This one just the generic "parent" crate with the abstractions
So there are implementations for almost every type like:
- JSON
- Proto
- XML

---

### serde_json (https://docs.rs/serde_json)
_Need to cover the `Value` type here as the XML needs to deserialize to it_


---

# Now we build

---

## Extras

### Clap
### Modules / testing
### Custom deserialize to get rid of `$text`