// Loops - iterate till condition met
pub fn run(){

// let mut count = 1;

  // Infinite without conditional obviously
  // loop{
  //   count += 1;
  //   println!("Number {}",count );

  //   if count == 20 {
  //     break;
  //   }
  // }

  //while loop
  //  while count <= 100 {
  //    if count % 15 == 0 {
  //      println!("FizzBuzz");
  //    } else if count % 3 == 0{
  //      println!("Fizz");
  //    } else if count % 5 == 0 {
  //      println!("Buzz");
  //    } else {
  //      println!("{}", count);
  //    }
  //    count += 1;
  //  }

   for x in 0..100{
     if x % 15 == 0 {
       println!("FizzBuzz");
     } else if x % 3 == 0{
       println!("Fizz");
     } else if x % 5 == 0 {
       println!("Buzz");
     } else {
       println!("{}", x);
     }
   }
}