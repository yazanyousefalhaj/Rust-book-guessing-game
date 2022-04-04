use std::io::Write;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    print!("Please input your guess: ");
    io::stdout().flush().unwrap();

    let random_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "exit" {
            break;
        }

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Please input a number: ");
                continue;
            }
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
