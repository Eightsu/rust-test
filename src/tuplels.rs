// A tuple is a group of values. Can be of mixed types.
//Maximum of twelve elements
//It's simple really.
// Tuples can be used as function arguments and as return values
// More info - https://doc.rust-lang.org/rust-by-example/primitives/tuples.html

pub fn run() {
  let person: (&str, &str, i8) = ("Jamar", "Powell", 26);

  println!("{}, {} is {}", person.0, person.1, person.2);
}