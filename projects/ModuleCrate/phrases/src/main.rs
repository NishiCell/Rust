extern crate phrases;

use phrases::english::{greetings, farewells};
use phrases::japanese;

fn main() {
    println!("Hello in English: {}", greetings::hello());
  println!("Goodbye in Englosh: {}", farewells::goodbye());

  println!("Hello in Japanese: {}", japanese::hello());
  println!("Goodbye in Japanese: {}", japanese::faaaaaa());
}