// VARIABLES ARE IMMUTABLE BY DEFAULT
// Variables hold primitives or references to data
// function scoped

pub fn run() {
  let name = "_sutra";

  // Explictly create mutable variables
  let mut age = 26;
  println!("my name is {} and I am currently {}", name, age);
  age = 1;
  println!("my name is {} and I am currently {}", name, age);

  //define Constants
  const ID: i32 = 001;
  println!("ID: {}", ID);

  //Defining multiple Variables
  let (_my_age, _my_location) = (26, "Kelowna");
  println!("{} in {}", _my_age, _my_location);

  
}
