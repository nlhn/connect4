use std::cmp;

use crate::board::Difficulty;
use crate::otto::TootOttoBoard;
use wasm_bindgen::prelude::*;
use web_sys::console;
#[wasm_bindgen]
pub struct OttoBot {
    depth: u32,
    play_as: char,
}

#[wasm_bindgen]
impl OttoBot {
    #[wasm_bindgen(constructor)]
    pub fn new(difficulty: Difficulty, ai_token: char) -> OttoBot {
        let depth = match difficulty {
            Difficulty::Easy => 3,
            Difficulty::Hard => 5,
        };
        let play_as = ai_token;

        OttoBot { 
            depth,
            play_as,
        }
    }

    #[wasm_bindgen]
    pub fn best_move(&self, board: &mut TootOttoBoard, player: char) -> String{
        let log = "OttoBot bestmove player: {} called".replace("{}", &player.to_string());
        console::log_1(&log.into());
        console::log_1(&"depth {}".replace("{}",&self.depth.to_string()).into());
        let (score, mov, mov_char) = self.minimax(board, self.depth, i32::MIN, i32::MAX, player);
        println!("Move: {} {}, Score: {}", mov, mov_char, score);
        let mov_str = mov.to_string() + mov_char.to_string().as_str();
        mov_str
    }

}

impl OttoBot {
    fn evaluate(&self, board: &mut TootOttoBoard, player: char) -> i32 {
        let mut score = 0;
        let opposite = if player == 'O' { 'T' } else { 'O' };
        let score_multiplier = 1;

        //score it similar to the has_winner function in TootOttoBoard
        for row in (0..board.height() as usize) {
            for col in (0..board.width() as usize){

                //give incentive to play in the middle
                if col == 3 || col == 2{
                    score += 1 * score_multiplier;
                }

                //check rows
                //make sure the board can be indexed
                if col + 3 < board.width() as usize{

                    //check for 4 in a row
                    if((board.get(row,col) == player)
                    && (board.get(row,col + 1) == opposite)
                    && (board.get(row,col + 2) == opposite)
                    && (board.get(row,col + 3) == player)){
                        score += 100000 * score_multiplier;
                    }

                    //check for 3 in a row
                    else if((board.get(row,col) == player)
                    && (board.get(row,col + 1) == opposite)
                    && (board.get(row,col + 2) == opposite)){
                        score += 100 * score_multiplier;
                    }

                    //check blocking
                    else if((board.get(row,col) == opposite)
                    && (board.get(row,col + 1) == player)
                    && (board.get(row,col + 2) == player)
                    && (board.get(row,col +3) == player)){
                        score += 500 * score_multiplier; //prioritize blocking over 3 in a row
                    }

                }

                if (row as i32) - 3 >= 0{
                    
                    //--------check vertical---------
                    //check for 4 in a row
                    if((board.get(row,col) == player)
                    && (board.get(row - 1,col) == opposite)
                    && (board.get(row - 2,col) == opposite)
                    && (board.get(row - 3,col) == player)){
                        score += 100000 * score_multiplier;
                    }

                    //check for 3 in a row
                    else if((board.get(row,col) == player)
                    && (board.get(row - 1,col) == opposite)
                    && (board.get(row - 2,col) == opposite)){
                        score += 100 * score_multiplier;
                    }

                    //check for 2 in a row
                    else if((board.get(row,col) == player)
                    && (board.get(row - 1,col) == opposite)){
                        score += 10 * score_multiplier;
                    }

                    //check blocking
                    else if((board.get(row,col) == opposite)
                    && (board.get(row - 1,col) == player)
                    && (board.get(row - 2,col) == player)
                    && (board.get(row - 3,col) == player)){
                        score += 500 * score_multiplier; //prioritize blocking over 3 in a row
                    }


                    //--------check up and right--------
                    if col + 3 < board.width() as usize{
                        //check for 4 in a row
                        if((board.get(row,col) == player)
                        && (board.get(row - 1,col + 1) == opposite)
                        && (board.get(row - 2,col + 2) == opposite)
                        && (board.get(row - 3,col + 3) == player)){
                            score += 100000 * score_multiplier;
                        }

                        //check for 3 in a row
                        else if((board.get(row,col) == player)
                        && (board.get(row - 1,col + 1) == opposite)
                        && (board.get(row - 2,col + 2) == opposite)){
                            score += 100 * score_multiplier;
                        }

                        //check for 2 in a row
                        else if((board.get(row,col) == player)
                        && (board.get(row - 1,col + 1) == opposite)){
                            score += 10 * score_multiplier;
                        }

                        //check blocking
                        else if((board.get(row,col) == opposite)
                        && (board.get(row - 1,col + 1) == player)
                        && (board.get(row - 2,col + 2) == player)
                        && (board.get(row - 3,col + 3) == player)){
                            score += 500 * score_multiplier; //prioritize blocking over 3 in a row
                        }
                    }

                    //check up and left
                    if col as i32 - 3 >= 0{
                        //check for 4 in a row
                        if((board.get(row,col) == player)
                        && (board.get(row - 1,col - 1) == opposite)
                        && (board.get(row - 2,col - 2) == opposite)
                        && (board.get(row - 3,col - 3) == player)){
                            score += 100000 * score_multiplier;
                        }

                        //check for 3 in a row
                        else if((board.get(row,col) == player)
                        && (board.get(row - 1,col - 1) == opposite)
                        && (board.get(row - 2,col - 2) == opposite)){
                            score += 100 * score_multiplier;
                        }

                        //check for 2 in a row
                        else if((board.get(row,col) == player)
                        && (board.get(row - 1,col - 1) == opposite)){
                            score += 10 * score_multiplier;
                        }

                        //check blocking
                        else if((board.get(row,col) == opposite)
                        && (board.get(row - 1,col - 1) == player)
                        && (board.get(row - 2,col - 2) == player)
                        && (board.get(row - 3,col - 3) == player)){
                            score += 500 * score_multiplier; //prioritize blocking over 3 in a row
                        }
                    }
                }

            }
        }
        score
    }

    fn minimax(&self, board: &mut TootOttoBoard, depth: u32,alpha: i32, beta: i32, player: char) -> (i32, u32, char) {
        let mut alpha = alpha;
        let mut beta = beta;
        let mut best_move = 2;
        let mut best_score = if player == 'O' { i32::MIN } else { i32::MAX };
        let mut best_move_char = player;

        if depth == 0 || board.is_terminal() {
           best_score = {
                if board.is_terminal(){
                    board.game_value()
                } else{
                    self.evaluate(board, self.play_as)
                }
           };
            return (best_score, 2, 'O'); //fight for otto
        }

        for i in (0..board.width() as u32) {

            console::log_1(&"AI Trying for a move".into());
            if board.allows_move(i) {

                let tokens = ['O', 'T'];
                for &token in tokens.iter() {
                    board.perform_move_plz(i, token, player);
                    let (score, _, _) = self.minimax(board, depth - 1,alpha, beta, if player == 'T' { 'O' } else { 'T' });
                    board.undo_move(i as usize);
                
                    if player == 'O' {
                        if score > best_score {
                            best_score = score;
                            best_move = i;
                            best_move_char = token;
                        }
                        alpha = cmp::max(alpha,best_score );
                    } else {
                        if score < best_score {
                            best_score = score;
                            best_move = i;
                            best_move_char = token;
                        }
                        beta = cmp::min(beta,best_score );
                    }

                    if beta <= alpha {
                        break;
                    }
                }
            }
        }

        (best_score, best_move, best_move_char)
    }
}