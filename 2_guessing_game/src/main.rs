use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // 1..=100 is range expression
    // a range expression in Rust includes lower and upper bound as well
    let secret_number  = rand::thread_rng().gen_range(1..=100);

    // we can place variable in the curly bracket to print instead of using format
    // comment out redundant printing
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // let statement is used to create a variable
        // mut statement is used to set the variable as mutable
        // rust variables are immutable by default
        // String is a string type from stl, and it's growable and UTF-8 encoded
        // :: is for accessing method of class
        // new() is a method of String type which is creating new String type instance
        let mut guess = String::new();

        io::stdin()
            // & indicates this argument is a reference, not as data itself
            // mut makes the passed guess mutable
            .read_line(&mut guess)
            .expect("Failed to read line");

        // different in-line form of match expression
        // parse() returns `Result` enum type that has `Ok` and `Err` variants
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // match expression
        // cmp method takes a reference, not a value
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
