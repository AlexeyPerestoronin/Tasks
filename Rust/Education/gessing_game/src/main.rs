use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    loop {
        println!("Input your guess:");

        let mut guess = String::new();
        let bite_count = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim().parse::<u32>().expect("Please type a number!");
        println!("Yor guess is: {guess} [{bite_count} bites obtained]");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}
