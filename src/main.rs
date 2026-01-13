// get the io library from the standard library
// this is kind of like namespaces in
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number game 🤔");
    let secret_number = rand::thread_rng().gen_range(1..10);

    loop {
        println!("Input your guess");

        // mut makes the variable reassignable/ mutable
        // by default they're immutable
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("type a number foo'");

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
