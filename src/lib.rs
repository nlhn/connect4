mod connect4;
mod board;
mod cli;
mod otto;
mod ottobot;

use crate::board::*;
use wasm_bindgen::prelude::*;
use crate::otto::TootOttoBoard;
use std::sync::{Mutex,Arc};
use lazy_static::lazy_static;

lazy_static!{
    //create an instance of the otto board
    static ref OTTO_BOARD: Mutex<Arc<TootOttoBoard>> = Mutex::new(Arc::new(TootOttoBoard::new(BoardSize::Standard)));
}


//----------------- OTTO interaction functions-----------------//

//intialize the board
#[wasm_bindgen]
pub fn toot_init_board(size: u32){
    //size = 0 -> standard size, size = 1 -> large size
    let mut board = OTTO_BOARD.lock().unwrap();
    match size {
        0 => *board = Arc::new(TootOttoBoard::new(BoardSize::Standard)),
        1 => *board = Arc::new(TootOttoBoard::new(BoardSize::Large)),
        _ => *board = Arc::new(TootOttoBoard::new(BoardSize::Standard)),
    }
}

//checks if a move is allowed, return true or false if the move is allowed
#[wasm_bindgen]
pub fn toot_allows_move(col: u32) -> bool {
    let board = OTTO_BOARD.lock().unwrap();
    board.allows_move(col as usize)
}

//A move is performed on the board and the backend board is updated
#[wasm_bindgen]
pub fn toot_perform_move(col: u32, token: char, player: char) {
    let mut board = OTTO_BOARD.lock().unwrap();
    let board_ref = Arc::get_mut(&mut board).unwrap();
    board_ref.perform_move(col as usize, token, player)
}

//checks if the game is over
#[wasm_bindgen]
pub fn toot_is_terminal() -> bool {
    let mut board = OTTO_BOARD.lock().unwrap();
    let board_ref = Arc::get_mut(&mut board).unwrap();
    board_ref.is_terminal()
}

//check if game has winner
#[wasm_bindgen]
pub fn toot_has_winner() -> char {
    let mut board = OTTO_BOARD.lock().unwrap();
    let board_ref = Arc::get_mut(&mut board).unwrap();
    board_ref.has_winner()
}

//get the winner of the game
#[wasm_bindgen]
pub fn toot_get_winner() -> char {
    let board = OTTO_BOARD.lock().unwrap();
    board.winner.unwrap_or_default()
}


