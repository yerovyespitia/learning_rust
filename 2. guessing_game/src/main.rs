use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // generates a random num in a range of 1 to 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess: ");

        // creates a mutable string variable
        let mut guess = String::new();

        // it lets you introduce an input and save it
        // the param takes the guess variable and assigned it to the new value
        // the read_line returns a Return value (Ok or Err)
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // it shadowing the mutable string variable to convert it to a num
        // parse returns a Result so we handle it with expected or match
        // in case return error it sends you to next iteration of the loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // it compares the guess u32 value to secret_number
        // if it is equal breaks the loop
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
