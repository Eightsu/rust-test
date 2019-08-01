


pub fn run() {
  greeting("hello", "Jane");

  let get_sum = add_sum( 6, 5);
  println!("the sum is: {}", get_sum);

  // A closure - Can use outside variables in closure.
  // Can also include outside variables
  let z: i32 = 34;

  let add_numbers = | n1: i32, n2: i32 | n1 + n2 + z;
  println!("cSum: {}", add_numbers(4,10)) // -> 48

}

// Simple function with mutable strings
fn greeting(greet: &str, name: &str) {

  println!("{} {} nice to meet ya!", greet, name);

}

fn add_sum(x: i32, y: i32) -> i32 { // the arrows tell the program we want a i32
  x + y // no semicolon lets the compliler know this is what we want to return.
}