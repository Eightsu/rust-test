// Most common interger - i32
/* Intergers: u8, u16, u32, u64, u128
Intergers:i8,i16,i32,i64,i128

*/

pub fn run() {
    // default - i32
    let _x = 1;

    // default - f64
    let _y = 2.5;

    //Add explicit type
    let _u: i128 = 913920390230;

    //Find max
    // If you need a number larger go higher
    println!("Max i32 {}", std::i32::MAX);
    println!("Max i32 {}", std::i64::MAX);

    //Setting a bool
    // Remember {:?} is debug https://doc.rust-lang.org/std/fmt/trait.Debug.html
    let is_active: bool = true;
    let greater_than = 5 > 20;
    println!("{:?}", is_active);
    println!("{:?}", greater_than);

    //Signify char through ''. Only one char allowed. More chars will show an error.

    let a = 'a';
    println!("{:?}", a);
}
