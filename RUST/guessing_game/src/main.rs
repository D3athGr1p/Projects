use rand::{self, Rng};
use std::io;

fn main() {
    let secret_num = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Enter your guess...");

        let mut num = String::new();

        io::stdin().read_line(&mut num).unwrap();

        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid Number!");
                eprintln!("Please type a number!");
                continue;
            }
        };

        match num {
            n if n < secret_num => println!("Too small!"),
            n if n > secret_num => println!("Too big!"),
            _ => {
                println!("You win!");
                break;
            }
        }
    }
}
