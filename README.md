# Corrosion - learning Rust

Stopped here: https://doc.rust-lang.org/book/ch03-02-data-types.html

## Running

`cargo run`

## Formatting

Run `cargo fmt` to format files.

## Resources

[official docs](https://doc.rust-lang.org/)

[crate registry](https://crates.io/)

https://product.letsgetrusty.com/strategy-training-vsl-eg?tid=T-0010&vid=V-0015

https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

## Notes

### The difference between `mut` and shadowing:

1. When you declare a variable with mut, it means that the variable's value can be changed after its initial assignment. This allows you to mutate the variable in place.
2. Shadowing allows you to declare a new variable with the same name as a previous variable, effectively "hiding" the previous variable. Each shadowed variable can have a different type and does not require `mut` for reassignment.

## Project ideas

1. Static site generator
2. Build a WebAssembly module that compiles Rust to WASM (then use it in a web app)
