
#![allow(dead_code)]
#![allow(unused_variables)]

//IMPORTS
mod utils;
mod tests;


use utils::*;
//use utils::{Position};
use tests::*;
const BOARD_SIZE: i32 = 9;





fn main() {
    test_board_game_logic();

}


fn interm() {

    let mut board = [0; 9];

    board[3] = 1;
    board[4] = 2;
    board[5] = 1;

    /*
    for (index, element) in board.iter_mut().enumerate() {
    *element = generate_random_range(0, 3);
    }
    */
    board_display(board);

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

    println!("{}", board_check_player_has_won(board, Position {x: 0, y: 0}, turn));
    println!("{}", board_check_player_has_won(board, Position {x: 0, y: 0}, turn));
}










/*
  UTILS:
*/

