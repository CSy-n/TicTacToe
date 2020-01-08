
use crate::utils::*;

/**
 Check if board Wins as expected
*/
pub fn test_board_game_logic() {
    let mut board = [0; 9];

    println!("Running Tests");
    
    board = [1,0,0, 0,0,0, 0,0,0];

    //println!("{}", board.len());

    // Expected PASS     
    board = [0,0,0, 0,0,0, 0,0,0];
    let htop = test_htop(board);
    
    println!("----------------------");        
    println!("test_htop => {}", htop);

    board_display(board);
    
    println!("----------------------");
    

}

pub fn test_htop(board: [i32; 9]) -> bool {
    //(board[0] == board[1]) && (board[0] == board[2])
    (board[0] == board[1]) && (board[0] == board[2])
}



fn reset_board() -> [i32; 9]{
    [0; 9]
}

/*

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
*/
