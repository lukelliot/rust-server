use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess a number between 1 and 100!");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  println!("Type it in below:");

  let mut guess = String::new();

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line!");

  match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Equal => println!("WINNER!"),
    Ordering::Equal => println!("Too big!!"),
  }

  println!("You guessed {}", guess)
}
