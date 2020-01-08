
use crate::utils::*;

/**
 Check if board Wins as expected
*/
pub fn test_board_game_logic() {
    let mut board = [0; 9];

    println!("Running Tests");
    println!("----------------------");
    


    board[3] = 1;
    board[4] = 2;
    board[5] = 1;
    //println!("{}", board.len());
    board_display(board);
}

fn reset_board() -> [i32; 9]{
    [0; 9]
}


