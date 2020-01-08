
#[derive(Debug, Copy, Clone)]
pub struct Position {
  pub x: i32,
  pub y: i32
}


impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "{{x: {}, y: {}}}", self.x, self.y)
    }
}



/*

Functions: 
    generate_random_range
    generate_random_position
    --------------------------
    board_cell_value
    board_current_player_turn
    board_position_within_bounds

    board_check_turn_valid
    board_is_corner
    board_is_mid_side
    board_is_side

    board_check_player_has_won
    ----------------------------
   



*/

/**
  Calculate current Players turn
  #Returns integer representation of player-turn
*/
pub fn board_current_player_turn(turn: i32) -> i32 {
  turn % 2
}


/**
  Check Validity of a turn. 
  - Checks Position is within Game board boundaries
  - Checks Cell is unoccupied
*/
pub fn board_check_turn_valid(board: [i32; 9], position: Position) -> bool {
  if board_position_within_bounds(position) && board_cell_value(board, position) == 0 {
    return true;
  }
  return false;
}

/**
    Generate arbitrary Random Integer within a Range.
*/
pub fn generate_random_range(start: i32, end: i32) -> i32 {
  use rand::prelude::*;
  let mut rng = thread_rng();
  return rng.gen_range(start, end);
}

/**
    Generate a Random Position
    - Valid on a Game Board (3x3)
*/
pub fn generate_random_position(x: i32, y: i32) -> Position {
  use rand::prelude::*;
  let mut rng = thread_rng();
  Position { x: rng.gen_range(0, 3), y: rng.gen_range(0, 3)}
}


/**
  Check if player has won
*/
pub fn board_check_player_has_won(board: [i32; 9], position: Position, turn: i32) -> bool{
    let player_id = board_cell_value(board, position);
    println!("player_id: {}", player_id);

    // HORIZONTALS
    if player_id == board[0] && player_id == board[1] && player_id == board[2] {
      println!("player_has_won  #h$1");
    } 

    if player_id == board[3] && player_id == board[4] && player_id == board[5] {
    println!("b[0:{}, 1:{}, 2:{}]", board[3], board[4], board[5]);

      println!("player_has_won #h$2");
    }

    if player_id == board[6] && player_id == board[7] && player_id == board[8] {
      println!("player_has_won  #h$3");
    }


    // VERTICALS
    if player_id == board[0] && player_id == board[3] && player_id== board[6] {
      println!("player_has_won  #V$1");
    }

    if player_id == board[1] && player_id == board[4] &&  player_id == board[7] {
      println!("player_has_won  #V$2");
    }

    if player_id == board[2] && player_id == board[5] && player_id == board[8] {
      println!("player_has_won  #V$3");
    }

    // DIAGONALS
    if player_id == board[0] && player_id == board[4] && player_id == board[8] {
      println!("player_has_won #d$1");
    }

    if player_id == board[2] && player_id == board[4] && player_id == board[6] {
      println!("player_has_won #D$2");
    }


  true
}





/**
  Checks the State of the Game
  - Complete
  - In Progress

   Checks if the current board configuration is 
*/
pub fn board_check_turn_result(board: [i32; 9], position: Position, turn: i32) -> i32 {
  
  //If the game is A Win, return 3
  //if the game is a draw return 2
  //if the game is a loss, return 1
  //If the game is still in progress return 0
    

  return 0;
  
}

/**
  Checks if the game board is filled
*/

pub fn board_is_filled(board: [i32; 9]) -> bool {
  for element in board.iter() {
    if *element != 0 {
      return false;
    }
  }
  return true;
}

/**
   Checks if the Position is a *side*
*/
pub fn board_is_side(position: Position) -> bool {
  if position.x == 1 && position.y == 1 {
    return true;
  }
  return false;
}

pub fn board_is_mid_side(position: Position) -> bool {
  if position.x == 1 && position.y == 0       //top
     || (position.x == 0 && position.y == 2)  //right
     || (position.x == 0 && position.y == 1)  //left
     || (position.x == 1 && position.y == 2) {//bottom
    return true;
  } 
  return false;
}

/**
  Checks is Position is a corner
*/
pub fn board_is_corner(position: Position) -> bool {
    if position.x == 0 && position.y == 0
     || (position.x == 2 && position.y == 0)
     || (position.x == 2 && position.y == 0)
     || (position.x == 2 && position.y == 2) {
        return true;
    } 
    return false;
}



/**
  Obtains value on a Board
  - Assumes valid Position 
*/
pub fn board_cell_value(board: [i32; 9], position: Position) -> i32 {
    board[(position.y * 3 + position.x) as usize]
}

/**
    
*/
pub fn board_position_within_bounds(position: Position) -> bool {
    if position.x >= 0 && position.x <= 3
        && position.y >= 0 && position.y <= 3 {
     return true;
    }
    return false;
}


/**
    Displays a textual representation of the Board.
*/
pub fn board_display(board: [i32; 9]) {

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


















