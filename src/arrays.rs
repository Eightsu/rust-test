// Arrays are fixed lists where all elements are the same data-type

// Import statement
use std::mem;

pub fn run() {
    // specified array number must be exact. if you say 6 give 6. data-type and len must be exact.
    let _numbers: [i32; 6] = [2, 3, 5, 1, 4, 6];

    // to print out the whole array u need to use the debug trait(?)
    println!("this is our array: {:?}", _numbers);

    // to get a single value
    println!("the first number in the array is {}", _numbers[0]);

    // can change specific numbers
    let mut _array2: [i32; 4] = [1, 2, 3,45];
    println!("this is our _array2: {:?}", _array2);

    // Starts as 3
    println!("{}",_array2[2]);

    _array2[2] = 10;

    //ends as 10.
    println!("{}", _array2[2]);

    // get array length
    println!("{}", _array2.len());

    // Returns the size of the pointed-to value in bytes. - froms docs
    println!("{}", mem::size_of_val(&_array2));

    // Slice -> [1,2,10,45]
    let slice: &[i32] =  &_array2;
    println!("Slice: {:?}", slice);
    
    // Slice -> [1,2,10]  .. means 
    let slice2: &[i32] =  &_array2[0..=2];
    println!("Slice: {:?}", slice2);
}
