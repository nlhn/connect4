mod connect4;
mod board;
mod cli;
mod otto;
mod ottobot;

use crate::connect4::{ Connect4Board, Connect4AI };
use cli::run_cli;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::{from_value, to_value};
use crate::board::*;
use std::i32;
use web_sys::console;
use wasm_bindgen::prelude::*;
use crate::otto::TootOttoBoard;
// use std::sync::{Mutex,Arc};
// use lazy_static::lazy_static;

lazy_static!{
    //create an instance of the otto board
    static ref OTTO_BOARD: Mutex<Arc<TootOttoBoard>>> = Mutex::new(Arc::new(TootOttoBoard::new(BoardSize::Standard)))
}


// #[derive(Serialize, Deserialize)]
// pub struct GameConfig {
//     height: u32,
//     width: u32,
//     last_row: Option<u32>,
//     last_col: Option<u32>,
//     board: Vec<Vec<char>>, 
//     last_player: Option<u32>, 
//     mode: u32,
//     game: String,
//     result: Option<u32>, // This could be null, so it's Option
// }


// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn bestMove(board: &JsValue, last_player: Option<u32>, game: String, last_col: Option<u32>, last_row: Option<u32>, mode: u32) -> u32 {
//     let board = to_vector(board.clone());
//     let board_size = get_board_size(board.clone(), game.clone());
//     let last_player = get_players(last_player.clone(), game.clone()).0;
//     let mut game = Connect4Board::new(board_size);
//     let ai = if mode == 1 {
//         Connect4AI::new(Difficulty::Easy)
//     } else {
//         Connect4AI::new(Difficulty::Hard)
//     };
//     game.set_board(board.clone());
//     (unimplemented!())
// }

// #[wasm_bindgen]
// pub fn best_move(game_config: &JsValue) -> Result<u32, JsValue> {
//     let config: GameConfig = from_value(game_config.clone())
//     .map_err(|e| JsValue::from_str(&format!("Error deserializing GameConfig: {:?}", e)))?;
//     let board_size = get_board_size(config.board.clone(), config.game.clone());
//     let (last_player, current_player) = get_players(config.last_player.clone(), config.game.clone());

//     // change this later to include Toot
//     let mut game = Connect4Board::new(board_size);
//     let ai = if config.mode == 1 {
//         Connect4AI::new(Difficulty::Easy)
//     } else {
//         Connect4AI::new(Difficulty::Hard)
//     };
    
//     game.set_board(config.board.clone());
//     game.set_last_col(config.last_col.clone());
//     game.set_last_row(config.last_row.clone());
//     game.set_last_player(last_player.clone());

//     let best_move = ai.best_move(&mut game, current_player.clone());

//     Ok(best_move)
// }

// #[wasm_bindgen]
// pub fn check_win(game_config: &JsValue) -> Result<Option<u32>, JsValue> {
//     let config: GameConfig = from_value(game_config.clone())
//     .map_err(|e| JsValue::from_str(&format!("Error deserializing GameConfig: {:?}", e)))?;
//     let board_size = get_board_size(config.board.clone(), config.game.clone());
//     let last_player = get_players(config.last_player.clone(), config.game.clone()).0;

//     let mut game = Connect4Board::new(board_size);
    
//     game.set_board(config.board.clone());
//     game.set_last_col(config.last_col.clone());
//     game.set_last_row(config.last_row.clone());
//     game.set_last_player(last_player.clone());

//     let result = if game.is_terminal() {
//         match game.game_value() {
//             i32::MAX => Some(0),
//             i32::MIN => Some(1),
//             _ => Some(2),
//         }
//     } else {
//         None
//     };

//     Ok(result)
// }

// #[wasm_bindgen]
// pub fn check_win(board: &JsValue, last_player: Option<u32>, game: String, last_col: Option<u32>, last_row: Option<u32>) -> Result<JsValue, JsValue> {
//     let board = to_vector(board.clone());
//     let board_size = get_board_size(board.clone(), game.clone());
//     let last_player = get_players(last_player.clone(), game.clone()).0;
//     let mut game = Connect4Board::new(board_size);

//     game.set_board(board.clone());
//     game.set_last_col(last_col.clone());
//     game.set_last_row(last_row.clone());
//     game.set_last_player(last_player.clone());

//     game.print();

//     console::log_1(&format!("performing a move").into());
//     game.perform_move(0, 'X');
//     game.print();

//     let bool = game.is_terminal();
    
//     let result = if bool {
//         console::log_1(&format!("is terminal").into());
//         match game.game_value() {
//             i32::MAX => JsValue::from(0),
//             i32::MIN => JsValue::from(1),
//             _ => JsValue::from(2),
//         }
//     } else {
//         console::log_1(&format!("not terminal").into());
//         JsValue::NULL
//     };
//     Ok(result)
// }



// pub fn to_vector(arr: JsValue) -> Vec<Vec<char>> {
//     let arrays: Vec<Vec<String>> = from_value(arr)
//         .unwrap_or_else(|_| panic!("Failed to convert from JsValue"));

//     arrays.iter().map(|inner| {
//         inner.iter().flat_map(|s| s.chars()).collect()
//     }).collect()
// }

// pub fn get_board_size(board: Vec<Vec<char>>, game: String) -> BoardSize {
//     let rows = board.len();
//     if game == "connect4" {
//         match rows {
//             6 => BoardSize::Standard,
//             7 => BoardSize::Large,
//             _ => BoardSize::Standard,
//         }
//     } else {
//         match rows {
//             4 => BoardSize::Standard,
//             6 => BoardSize::Large,
//             _ => BoardSize::Standard,
//         }
//     }
// }

// pub fn get_players(player: Option<u32>, game: String) -> (Option<char>, char) {
    
//     if game == "connect4" {
//         match player {
//             None => (None, 'X'),
//             Some(p) => {
//                 match p {
//                     0 => (Some('X'), 'O'),
//                     1 => (Some('O'), 'X'),
//                     _ => (None, 'X'),
//                 }
//             }
//         }
//     } else {
//         match player {
//             None => (None, 'T'),
//             Some(p) => {
//                 match p {
//                     0 => (Some('T'), 'O'),
//                     1 => (Some('O'), 'T'),
//                     _ => (None, 'T'),
//                 }
//             }
//         }
//     }
// }


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


