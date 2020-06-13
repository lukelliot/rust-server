use std::io;
use rand::Rng;

fn main() {
  println!("Guess a number between 1 and 100!");

  let secret_number = rand::thread_rng().gen_range(1, 100)

  println!("Type it in below:");

  let mut guess = String::new();

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line!");

  println!("You guessed {}", guess)
}
