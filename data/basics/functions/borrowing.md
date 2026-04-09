# Borrowing in Rust

Borrowing allows references without taking ownership.

## Example
```rust
fn print_str(s: &String) {
    println!("{}", s);
}
Practice

Create a function that takes a reference and prints it.
