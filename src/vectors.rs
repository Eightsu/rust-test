// Vectors are fixed lists where all elements are the same data-type

// Import statement

pub fn run() {

  // A little different from the array.
    let mut _numbers: Vec<i32> = vec![2, 3, 5, 1, 4, 6];

    _numbers.push(16);

    // must use debug to print all
    println!("{:?}", _numbers);

    println!("{}", _numbers[3]);

    // to remove vector
    _numbers.pop();
    println!("{:?}", _numbers);

    //loop through vector values 

    for x in _numbers.iter(){
      println!("number:{}", x)
    }

    //loop & mutate

    for x in _numbers.iter_mut(){
      *x *=2; // Here we are multiplying each vector in numbers by 2.
    }
    println!("{:?}", _numbers);
}
