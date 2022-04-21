use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101); //Start is inclusive, end is exclusive. To have an inclusive end : (1..=100)

    //println!("The secret number is: {}", secret_number); //secret number is per default an i32

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //let guess: u32 = guess.trim().parse().expect("Please type a number"); //: u32 specify to rust the type. Trim will remove whitespace on a string to obtain numerical data.
        //For ex: read_line required a enter from user that include \n
        //This one will be removed. Parse method on a string partes a string into some kind of number. We need to tell Rust the exact number that we want.
        //There we specified with : u32

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

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
