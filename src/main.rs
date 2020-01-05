
fn main() {
  let mut board = [0; 9];
  //board.iter().for_each(|e| { e = &2; println!("{}", e)} );

  for (index, element) in board.iter_mut().enumerate() {
    *element = generate_random_range(0, 2);
  }

  display_board(board);

}


fn display_board(board: [i32; 9]) {

    for (index, mut element) in board.iter().enumerate() {
      print!("| {} ", element);

      if (index + 1 )% 3 == 0 {
        println!("");
      }
      
    }
}



const BOARD_SIZE: i32 = 9;

struct Position {
  x: i32,
  y: i32
}



impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "{{x: {}, y: {}}}", self.x, self.y)
    }
}

fn generate_random_position() -> Position {
  use rand::prelude::*;
  let mut rng = thread_rng();
  return Position { x: rng.gen_range(0, SIZE), y: rng.gen_range(0, SIZE) };
}


fn generate_random_range(start: i32, end: i32) -> i32 {
  use rand::prelude::*;
  let mut rng = thread_rng();
  return rng.gen_range(start, end);
}


/*
  Display Random Positions:
  for i in 0..10 {
    println!("{}", generate_random_position());
  }
*/


/*
  UTILS:
*/

