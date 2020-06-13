use std::io;

fn main() {
  println!("Guess a number?");

  println!("Type it in below:");

  let mut guess = String::new();

  io::stdin()
    .read_line(buf: &mut String)
    .expect("Failed to read line!")

}
