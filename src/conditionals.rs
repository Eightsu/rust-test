// if, else and all that.
// This example is very simple, but yeah.
// !there's no ternary operator.

pub fn run(){
  let age: u8 = 22;
  let check_id: bool = false;
  let already_knows_age: bool = false;

  //if else && = AND  || = OR
  if age >= 21 && check_id || already_knows_age {
    println!("cheers! have a beer!");
  } else if age < 21 && check_id  {
    println!("get out.");
  } else {
    println!("i gotta see your id.")
  }

  // if shorthand
  let is_of_age: bool =  age >=21;
  println!("is of age? {}", is_of_age);



}