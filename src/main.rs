

//IMPORTS
mod utils;

use utils::*;



const BOARD_SIZE: i32 = 9;

#[derive(Debug, Copy, Clone)]
struct Position {
  x: i32,
  y: i32
}




fn main() {
  let mut board = [0; 9];

  for (index, element) in board.iter_mut().enumerate() {
    *element = generate_random_range(0, 3);
  }

  display_board(board);

  board.iter_mut().enumerate().for_each(|(i,e)| { 
      print!("{}", e);
      if (i + 1 ) % 3 == 0 {
        println!("");
      }

  });

  println!("----------------------------\n");
  let mut new_board = [0; 9];
  let mut turn = 0;
  println!("{}", new_board.len());

  println!("{}", board_check_turn_valid(board, Position {x: 0, y: 0}));
  
  println!("{}", board_check_turn_result(board, Position {x: 0, y: 0}, turn));
  println!("{}", turn);
}


/**
  Checks the State of the Game
  - Complete
  - In Progress

   Checks if the current board configuration is 
*/
fn board_check_turn_result(board: [i32; 9], position: Position, turn: i32) -> i32 {
  
  //If the game is A Win, return 3
  //if the game is a draw return 2
  //if the game is a loss, return 1
  //If the game is still in progress return 0
    

  return 0;
  
}

/**
  Checks if the game board is filled
*/

fn board_is_filled(board: [i32; 9]) -> bool {
  for element in board.iter() {
    if element != 0 {
      return false;
    }
  }
  return true;
}


/**
  Check 
*/

/**
  Calculate current Players turn
  #Returns integer representation of player-turn
*/
fn board_current_player_turn(turn: i32) -> i32 {
  turn % 2
}


/**
  Check Validity of a turn. 
  - Checks Position is within Game board boundaries
  - Checks Cell is unoccupied
*/
fn board_check_turn_valid(board: [i32; 9], position: Position) -> bool {
  if position_within_bounds(position) && board_cell_value(board, position) == 0 {
    return true;
  }
  return false;
}

fn board_cell_value(board: [i32; 9], position: Position) -> i32 {
  board[(position.y * 3 + position.x) as usize]
}

fn position_within_bounds(position: Position) -> bool {
  if position.x >= 0 && position.x <= 3
    && position.y >= 0 && position.y <= 3 {
    return true;
  }
  return false;
}



fn display_board(board: [i32; 9]) {

    for (index, element) in board.iter().enumerate() {

      if *element == 0 {
        print!("| {} ", " ");
      } 
      else if *element == 1 {
        print!("| {} ", "X");

      } else {
        print!("| {} ", "O");
      }      

      if (index + 1 ) % 3 == 0 {
        println!("");
      }
      
    }
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
  let random_number = generate_random_range(0, BOARD_SIZE) as f64;
  
  return Position { x: (random_number / 3f64).floor() as i32, y: (random_number % 3f64) as i32};
}



/*
  UTILS:
*/

