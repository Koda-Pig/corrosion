# Corrosion - learning Rust

Stopped here: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#statements-and-expressions

## Running

`cargo run`

## Formatting

Run `cargo fmt` to format files.

## Resources

[official docs](https://doc.rust-lang.org/)

[crate registry](https://crates.io/)

https://product.letsgetrusty.com/strategy-training-vsl-eg?tid=T-0010&vid=V-0015

https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

https://learn.letsgetrusty.com/starter-pack/scheduled/

https://rustlings.rust-lang.org/

## Notes

### The difference between `mut` and shadowing:

1. When you declare a variable with mut, it means that the variable's value can be changed after its initial assignment. This allows you to mutate the variable in place.
2. Shadowing allows you to declare a new variable with the same name as a previous variable, effectively "hiding" the previous variable. Each shadowed variable can have a different type and does not require `mut` for reassignment.

### General thoughts

I'm most interested in building projects in Rust that can be compiled to Web Assembly.

## Project ideas

1. Static site generator
2. Build a WebAssembly module that compiles Rust to Wasm (then use it in a web app)
3. Smart contracts. Solana contracts are mostly written in Rust. Solana itself is mostly built in Rust.
   1. https://github.com/solana-labs

## Top 4 Rust mistakes (according to Let's Get Rusty)

1. Thinking you can write Rust code the same way you do as in other languages.
2. Neglecting the most important 20% of Rust, instead trying to learn everything at once.
3. Being a productive procrastinator.
4. Trying to vibe code Rust like it's JS.
