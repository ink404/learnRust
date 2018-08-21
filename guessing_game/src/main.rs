extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess The number!");

    let secret_num = rand::thread_rng().gen_range(1, 100);
    // println!("The secret num is: {}", secret_num);

    println!("Input a number: ");
    // variables immutable by default - have to delcare mut
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to readline!");
    println!("You guessed {}", guess);

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Guess too small!"),
        Ordering::Equal => println!("WINNER!"),
        Ordering::Greater => println!("Guess too big!"),
    }

    println!("Secret was {}", secret_num);
}
