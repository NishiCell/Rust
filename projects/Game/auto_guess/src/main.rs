extern crate rand;

use std::cmp::Ordering;
use rand::Rng;

fn main() {

    auto_guess();
}

fn auto_guess(){
    println!("Guess the number!");

    let secret_number = 
    rand::thread_rng().gen_range(1, 101);
    
    println!("Please input your guess.");

    let mut guess = rand::thread_rng().gen_range(1, 101);

    loop{


        println!("you guessed: {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => {
                println!("too small!");
                if guess > 85 {
                    guess += 1;
                } else {
                    guess += rand::thread_rng().gen_range(1, 15);
                }
            }
            Ordering::Greater => {
                println!("too big!");
                if guess > 15{
                    guess -= rand::thread_rng().gen_range(1, 15);
                } else {
                    guess -= 1;
                }
            }
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
