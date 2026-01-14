// get the io library from the standard library
// this is kind of like namespaces in
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number game 🤔");

    let secret_number = rand::thread_rng().gen_range(1..10);

    // some tuples
    let unrelated_tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = unrelated_tuple; // destructured with pattern + prefixed with _ to ignore the fact that it's an unused var
    let _last_one = unrelated_tuple.2; // access an element in the tuple by its index

    // some arrays
    // every element in an array has to have the same type, and the array has a fixed length
    const _UNRELATED_ARRAY: [i32; 5] = [1, 2, 8000, 99, 420];

    loop {
        println!("Input your guess");

        // mut makes the variable reassignable/ mutable
        // by default they're immutable
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You can do better than that m8. Try again");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
