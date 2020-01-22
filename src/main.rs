
#![allow(dead_code)]
#![allow(unused_variables)]

//IMPORTS
mod utils;
mod tests;


use utils::*;
use tests::*;

//use utils::{Position};



use std::io::{self, Read, Write};
use std::cell::RefCell;
use std::rc::Rc;

use termion::event::Key;
use termion::{style, color};

use gameboard::{Game, InputListener, Cursor, Cell, ResourceTable};

fn create_resources() -> ResourceTable {
    let mut res = ResourceTable::new();
    res.insert(0, String::from("  OO   O  O   OO  "));
    res.insert(1, String::from(" X  X   XX   X  X "));
    res
}

struct App {}

impl<R: Read, W: Write> InputListener<R, W> for App {
    fn handle_key(&mut self, key: Key, game: &mut Game<R, W, Self>) {
        match key {
            Key::Char('q') => game.stop(),
            _ => {}
        }
    }
}

fn main() {
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let app = Rc::new(RefCell::new(App {}));

    let cursor = Cursor::new(color::Rgb(0, 0, 200), gameboard::Position(0, 0), true, None);
    let mut board = gameboard::Board::new(3, 3, 6, 3, true, Some(create_resources()));
    board.init_from_vec(
        &vec![
            Cell::Empty,
            Cell::ResourceId(0),
            Cell::ResourceId(1),
            Cell::Char('z'),
            Cell::Char('▒'),
            Cell::Content(
                format!("{}aaaaaaaa{}aaaaaaaaaa",
                        color::Fg(color::Red),
                        color::Fg(color::Blue))
            ),
            // this cell breaks cursor highlighting
            Cell::Content(
                format!("{}bbb{}bbbbb{}bbbb{}bbb{}bbb",
                        color::Fg(color::Red),
                        style::Bold,
                        style::Reset,
                        color::Fg(color::Blue),
                        style::Reset)
            ),
            // this cell breaks cursor highlighting
            Cell::Content(
                format!("{}cccccccccccc{}cccccc",
                        color::Bg(color::Red),
                        style::Reset)
            ),
            Cell::Content(
                format!("{}dddddddd{}dddddddddd",
                        color::Fg(color::Red),
                        style::Bold)
            )],
        Some(cursor));
    let game = Rc::new(RefCell::new(Game::new(stdin, stdout, Rc::clone(&app))));
    game.borrow_mut().init(board, None);
    game.borrow_mut().start();
}




fn game_board_logic() {
    // Create game board
    //let mut board = [0; 9];

    let mut board = Board::new();

    let selection = Position {x: 0, y: 0};

     // Game Board has no marks on it...
    //Display game board
    board.display();    
    //board_display(&board);


    // Place piece of game board [Correct piece]
    board.place_piece(2, Position {x: 0, y: 2});
    board.place_piece(2, Position {x: 1, y: 2});
    board.place_piece(2, Position {x: 2, y: 2});


    // board.place_piece(2, Position {x: 0, y: 0});
    // board.place_piece(2, Position {x: 1, y: 0});
    // board.place_piece(2, Position {x: 2, y: 0});

    board.place_piece(1, Position {x: 0, y: 1});
    board.place_piece(1, Position {x: 1, y: 1});
    board.place_piece(1, Position {x: 2, y: 1});
    //let result = board.take_turn(Position {x: 2, y: 0});
    board.increment_counter();
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

