// pointers refs - point to a resource in memory

pub fn run() {

  let arr1 = [1,2,3];
  let arr2 = arr1;

  println!("Values: {:?}", (arr1, arr2)); // -> [1,2,3], [1,2,3]

  // what if we use vectors?

  // With non-primitives, if you assign another variable to a piece of data 
  // The first value loses its hold on it.
  // You'll need to use a reference (&) and point to the resource.

  let vect1 = vec![1,2,3];
  let vect2 = &vect1;  // gotta reference it!

  println!("Vec: {:?}",(&vect1, vect2) ); // again! down here!
     
     let reference = &4;
     match reference {

        val => println!("Got a value via destructuring: {:?}", val),
    }

}