
//* std::env  More info on args https://doc.rust-lang.org/std/env/index.html

use std::env;

pub fn run(){
  let args: Vec<String> = env::args().collect();

  let command = args[1].clone();

  // gives path back plus any arguments passed to it.
  println!("Args {:?}", args);

  // What does command end up being? 
  println!("Command: {}", command);

  if command == "hello" {
    println!("Nice to meet you User!");
  }

}