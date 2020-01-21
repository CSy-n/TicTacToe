
use crate::utils::*;

/**
 Check if board Wins as expected
*/
pub fn test_board_game_logic() {
    let board = [0; 9];

    println!("Running Tests...");
    
    let board = [1,0,0, 0,0,0, 0,0,0];

    //println!("{}", board.len());


  //  horizontal_tests();
//    vertical_tests();
    diagonal_tests();




     
    
    

}

fn horizontal_tests() {
    println!("______________________");        
    println!("Horizontal Tests");
    println!("______________________");   


     
    
    // Expected PASS     
    let board = [1,1,1, 0,0,0, 0,0,0];
    let htop = test_htop(board);

    display_results(board, "test_htop", htop);


    // Expected PASS     
    let board = [0,0,0, 1,1,1, 0,0,0];
    let hmid = test_hmid(board);

    display_results(board, "test_hmid", hmid);

    // Expected PASS     
    let board = [0,0,0, 0,0,0, 1,1,1];
    let hbot = test_hbot(board);

    display_results(board, "test_hbot", hbot);

   //-----------------------------------

    // Expected FAIL     
    let board = [1,0,1, 0,0,0, 0,0,0];
    let htop = test_htop(board);

    display_results(board, "test_htop", htop);


    // Expected FAIL     
    let board = [0,0,0, 1,0,1, 0,0,0];
    let hmid = test_hmid(board);

    display_results(board, "test_hmid", hmid);

    // Expected FAIL
    let board = [0,0,0, 0,0,0, 1,0,1];
    let hbot = test_hbot(board);

    display_results(board, "test_hbot", hbot);

}

fn vertical_tests() {





    println!("______________________");        
    println!("VERTICAL Tests");
    println!("______________________");   


     
    
    // Expected PASS    
    let board = [1,0,0, 1,0,0, 1,0,0];
    let htop = test_vleft(board);

    display_results(board, "test_vleft", htop);


    // Expected PASS     
    let board = [0,1,0, 0,1,0, 0,1,0];
    let hmid = test_vmid(board);

    display_results(board, "test_vmid", hmid);

    // Expected PASS     
    let board = [0,0,1, 0,0,1, 0,0,1];
    let hbot = test_vright(board);

    display_results(board, "test_vright", hbot);



   //-----------------------------------
    // Expected FAIL    
    let board = [1,0,0, 0,0,0, 1,0,0];
    let htop = test_htop(board);
    display_results(board, "test_vleft", htop);


    // Expected FAIL     
    let board = [0,1,0, 0,0,0, 0,1,0];
    let hmid = test_hmid(board);

    display_results(board, "test_vmid", hmid);


    // Expected FAIL     
    let board = [0,0,1, 0,0,0, 0,0,1];
    let hbot = test_hbot(board);

    display_results(board, "test_vright", hbot);
}

fn diagonal_tests() {
    println!("______________________");        
    println!("DIAGONAL Tests");
    println!("______________________");   


     
    
    // Expected PASS    
    let board = [1,0,0, 0,1,0, 0,0,1];
    let htop = test_dleft(board);

    display_results(board, "test_dleft", htop);

    // Expected PASS    
    let board = [0,0,1, 0,1,0, 1,0,0];
    let htop = test_dright(board);

    display_results(board, "test_dright", htop);


   //=============================================


    // Expected FAIL    
    let board = [1,0,0, 0,0,0, 0,0,1];
    let htop = test_dleft(board);

    display_results(board, "test_dleft", htop);

    // Expected FAIL    
    let board = [0,0,1, 0,0,0, 1,0,0];
    let htop = test_dright(board);

    display_results(board, "test_dright", htop);
}






fn display_results(board: [i32; 9], test: &str, result: bool) {
    println!("----------------------");        
    println!("{} => {}", test, result);

    board_display(board);
    
    println!("----------------------");
}

//Diagonals
pub fn test_htop(board: [i32; 9]) -> bool {
    (board[0] != 0) && (board[0] == board[1]) && (board[0] == board[2])
}

pub fn test_hmid(board: [i32; 9]) -> bool {
    (board[3] != 0) && (board[3] == board[4]) && (board[3] == board[5])
}


pub fn test_hbot(board: [i32; 9]) -> bool {
    (board[6] != 0) && (board[6] == board[7]) && (board[6] == board[8])
}


//Verticals

pub fn test_vleft(board: [i32; 9]) -> bool {
    (board[0] != 0) && (board[0] == board[3]) && (board[0] == board[6])
}

pub fn test_vmid(board: [i32; 9]) -> bool {
    (board[1] != 0) && (board[1] == board[4]) && (board[1] == board[7])
}

pub fn test_vright(board: [i32; 9]) -> bool {
    (board[2] != 0) && (board[2] == board[5]) && (board[2] == board[8])
}


// Diagonals

pub fn test_dleft(board: [i32; 9]) -> bool {
    (board[0] != 0) && (board[0] == board[4]) && (board[0] == board[8])
}

pub fn test_dright(board: [i32; 9]) -> bool {
    (board[2] != 0) && (board[2] == board[4]) && (board[2] == board[6])
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
