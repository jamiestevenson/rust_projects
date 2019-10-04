use std::io;
use std::cmp::Ordering;
use rand::Rng;

// Notes
// String is growable
// The :: notation indicates a 'static' method on a type, not an instance

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number); //debug

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // Mutable var, type is inferred.

    // Could also write std::io::stdin without import above
        io::stdin().read_line(&mut guess) // & is reference (not copy), mut is mutable
            .expect("Failed to read line"); // Result.Err handler, returns OK to var or crashes with error string.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(x) => {
                println!("Oops: {}", x);
                continue;
            },
        };
        //.expect("Please type a number"); // Shadows previous ''guess' variable use

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // breaks out the containing loop.
            }
        }
    }
}
