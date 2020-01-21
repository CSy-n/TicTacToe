
#![allow(dead_code)]
#![allow(unused_variables)]

//IMPORTS
mod utils;
mod tests;


use utils::*;
//use utils::{Position};
use tests::*;

fn main() {
    // Test Game Logic:
    //test_board_game_logic();


    // Create game board
    //let mut board = [0; 9];

    let mut board = Board::new();
    let turn = 0;
    let selection = Position {x: 0, y: 0};

     // Game Board has no marks on it...
    //Display game board
    board.display();    
    //board_display(&board);


    // Place piece of game board [Correct piece]
    board.place_piece(2, Position {x: 0, y: 0});
    board.place_piece(2, Position {x: 1, y: 0});
    board.place_piece(2, Position {x: 2, y: 0});

    board.place_piece(2, Position {x: 0, y: 1});
    board.place_piece(2, Position {x: 1, y: 1});
    board.place_piece(2, Position {x: 2, y: 1});
    //let result = board.take_turn(Position {x: 2, y: 0});
    let result = board.check_turn_result();

    // Check if piece was a winning move.
    // Update state of `turn'
//    game_check_turn_result(board, selection, turn);
    //board.check_turn_result(selection, turn);
    

    //Display game board...
    println!("-----------------------------");
    //board_display(&board);
    board.display();  
    println!("Board-turn: {}; result=> {}", board.turn, result);




}


fn interm() {

    //let mut board = [0; 9];
    let mut board = Board::new();



    //board[3] = 1;
    //board[4] = 2;
    //board[5] = 1;

    /*
    for (index, element) in board.iter_mut().enumerate() {
    *element = generate_random_range(0, 3);
    }
    */
    board.display();

    board.board.iter_mut().enumerate().for_each(|(i,e)| { 
      print!("{}", e);
      if (i + 1 ) % 3 == 0 {
        println!("");
      }

    });

    //println!("----------------------------\n");
    //let mut new_board = [0; 9];
    //let mut turn = 0;
    //println!("{}", new_board.len());

    //println!("{}", board_check_turn_valid(board.board, Position {x: 0, y: 0}));

    //println!("{}", board_check_player_has_won(board.board, Position {x: 0, y: 0}, turn));
   // println!("{}", board_check_player_has_won(board.board, Position {x: 0, y: 0}, turn));
}










/*
  UTILS:
*/

