// Enums are types which have a few definite values(?) more info - https://doc.rust-lang.org/rust-by-example/custom_types/enum.html?highlight=enums#enums

enum Movement {

  // Variants
  Up,
  Down,
  Left,
  Right
}

fn move_avatar(m: Movement){
  // Perform action dependant on movement. Use Match (Which is kinda like switch.)
  match m {
    Movement::Up => println!("Avatar has moved Up!"),
    Movement::Down => println!("Avatar has moved Down!"),
    Movement::Left => println!("Avatar has moved Left!"),
    Movement::Right => println!("Avatar has moved Right!")
  }
}

pub fn run(){

  // Attatch each to a variable
  let av_up = Movement::Up;
  let av_down = Movement::Down;
  let av_left = Movement::Left;
  let av_right = Movement::Right;

  //Invocation
  move_avatar(av_up);
  move_avatar(av_down);
  move_avatar(av_left);
  move_avatar(av_right);

}