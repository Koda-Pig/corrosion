// get the io library from the standard library
// this is kind of like namespaces in
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// this is a 'statement' function (doesn't return anything)
fn statement_function(some_text: &str, single_char: char) {
    println!("I like snake case. Snake case is nice. Here's an argument: {single_char}s");
    println!("{some_text}");
}

// this is an 'expression' function, that does
fn expression_function(x: u32) -> u32 {
    return x / 2;
}

// now this is unusual behavior to me:
fn strange_indeed() {
    let y = {
        let x = 3;
        // if it has a semicolon, it becomes a statement.
        // x + 1;
        // without a semicolon, it's an expression (returns something)
        x + 1
    };
    println!("The value of y is: {y}");
}

// here's a simpler version of this:
// valid expression:
// fn add(a: u32, b: u32)->u32 {a+b}
// INvalid expression:
// fn add(a: u32, b: u32)->u32 {a+b;}

fn main() {
    statement_function("kakaka", '🐷');
    expression_function(22);
    strange_indeed();

    println!("Guess the number game 🤔");

    let secret_number = rand::thread_rng().gen_range(1..10);

    // some tuples
    let unrelated_tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = unrelated_tuple; // destructured with pattern + prefixed with _ to ignore the fact that it's an unused var
    let _last_one = unrelated_tuple.2; // access an element in the tuple by its index

    // some arrays
    // every element in an array has to have the same type, and the array has a fixed length
    // writtedn with the type of each elem, and the number of elems in the arr
    const _UNRELATED_ARRAY: [i32; 5] = [1, 2, 8000, 99, 420];

    // accessed as you would expect:
    let _first_elem = _UNRELATED_ARRAY[0];
    // let _invalid_elem = _UNRELATED_ARRAY[99]; // this would throw an error

    // println!("invalid probs: {_invalid_elem}");

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
