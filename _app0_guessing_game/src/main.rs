use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {secret_num}");

    loop {
        println!("Input you number.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        }
    }


}
