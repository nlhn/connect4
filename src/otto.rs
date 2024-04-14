use std::{collections:: HashSet, fmt};
use super::board::*;
use std::io::{self, Write};
use crate::ottobot;
use wasm_bindgen::prelude::*;
use web_sys::console;

///Player interacts directly with the board
///and the board interacts with the bot where the bot will
///"return" a move to place on the board
///
/// The board will keep track of the game state and the bot 

#[wasm_bindgen]
pub struct TootOttoBoard {
    board: Vec<Vec<char>>,

    //for custom size
    width: u32,
    height: u32,

    //winning cond checkers
    last_player: Option<char>, //'o' for otto and 't' for toot
    last_row: Option<u32>,
    last_col: Option<u32>,

    winner: Option<char>,
}

#[wasm_bindgen]
impl TootOttoBoard {
    #[wasm_bindgen(constructor)]
    pub fn new(size: BoardSize) -> TootOttoBoard {
        let (width, height) = match size {
            BoardSize::Standard => (6, 4),
            BoardSize::Large => (9, 6),
        };

        TootOttoBoard {
            width,
            height,
            board: vec![vec![' '; width as usize]; height as usize],
            last_row: None,
            last_col: None,
            last_player: None, 
            winner: None,
        }

    }

    #[wasm_bindgen]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[wasm_bindgen]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[wasm_bindgen]
    pub fn last_row(&self) -> Option<u32> {
        self.last_row
    }

    #[wasm_bindgen]
    pub fn last_col(&self) -> Option<u32> {
        self.last_col
    }

    #[wasm_bindgen]
    pub fn get_winner(&self) -> Option<char> {
        self.winner
    }

    
    ///Takes a column and a token and places the token on that column
    #[wasm_bindgen]
    pub fn perform_move_plz(&mut self, col: u32, tok: char, player: char) {
        //println!("Player: {}, Token: {}, Column: {}", player, tok, col);
        // decrement from the bottom row to the top row
        for row in (0..self.height).rev(){
            if self.board[row as usize][col as usize as usize] == ' ' {
                self.board[row as usize][col as usize as usize] = tok;
                self.last_row = Some(row);
                self.last_col = Some(col);
                self.last_player = Some(player);
                break;
            }
        }
    }

    #[wasm_bindgen]
    pub fn is_terminal(&mut self) -> bool {
        if (self.has_winner() !='f') || self.is_draw() {
            return true;
        }
        return false;
    }

    #[wasm_bindgen]
    pub fn is_draw(&self) -> bool {
        self.available_moves().is_empty()
    }

    
    ///Takes a board and checks if the board is in a winning position
    ///returns "w" if there is a winner
    /// "t" if the game is a tie
    /// "f" if the game is still in progress
    #[wasm_bindgen]
    pub fn has_winner(&mut self) -> char {

        let row = self.last_row;
        let col = self.last_col;

        //no move made so far
        if row.is_none() || col.is_none(){
            return 'f';
        }

        //beware that either player could win so we need to check both

        //check for horizontal win
    
        let rows = self.height;
        let cols = self.width;
        let mut winners_set = HashSet::new();

        //check every row and col
        for row in (0..rows).rev() {
            for col in 0..cols {
                let token = self.board[row as usize][col as usize];

                if token == ' '{
                    continue;
                }

                //match to opposite string instead
                let opposite: char = match &token {
                    'O' => 'T',
                    'T' => 'O',
                    _ => continue, //skip if empty
                };

                //println!("{}, {}, {}", token, row, col);

                // Check right
                if col + 3 < cols
                    && (self.board[row as usize][col as usize + 1] == opposite)
                    && (self.board[row as usize][col as usize + 2] == opposite)
                    && (self.board[row as usize][col as usize + 3] == token)
                {
                    winners_set.insert(token);
                    continue;
                }

                if (row as i32) - 3 >= 0 {
                    // Check up
                    if(self.board[row as usize -1][col as usize] == opposite)
                        &&(self.board[row as usize -2][col as usize] == opposite)
                        &&(self.board[row as usize -3][col as usize] == token)
                    {
                        winners_set.insert(token);
                        continue;
                    }

                    // Check up and right
                    if col  + 3 < cols
                        && (self.board[row as usize -1][col as usize+1] == opposite)
                        && (self.board[row as usize -2][col as usize+2] == opposite)
                        && (self.board[row as usize -3][col as usize+3] == token)
                    {
                        winners_set.insert(token);
                        continue;
                    }

                    // Check up and left
                    if (col as i32) - 3 >= 0
                        && (self.board[row as usize -1][col as usize-1] == opposite)
                        && (self.board[row as usize -2][col as usize-2] == opposite)
                        && (self.board[row as usize -3][col as usize-3] == token)
                    {
                        winners_set.insert(token);
                        continue;
                    }
                }
            }
        }

        match winners_set.len() {
            0 => 'f',
            1 => {
                //get the char from the winners set
                let winner = winners_set.iter().next().unwrap();
                
                //set board.winner to the winner
                self.winner = Some(*winner);
                return 'w';
            },
            _ => 't',
        }
    }

    ///Takes a column and reutrns true if a move can be made into
    ///that column. False otherwise
    #[wasm_bindgen]
    pub fn allows_move(&self, col: u32) -> bool{
        col < self.width() as u32 && self.board[0][col as usize] == ' '
    }
}

impl TootOttoBoard {

    /// Returns a list(HashSet) of available moves on the board
    pub fn available_moves(&self) -> Vec<u32>{
        let mut moves = Vec::new();
        for col in 0..self.width {
            if self.allows_move(col) {
                moves.push(col);
            }
        }
        return moves;
    }


    ///Takes a row and column and returns the token at that position
    pub fn get(&self, row: usize, col: usize) -> char {
        self.board[row][col]
    }

    ///Takes a column and removes the token from that column
    /// this is used to undo a move (for AI)
    pub fn undo_move(&mut self, col: usize) {
        for row in 0..self.height {
            if self.board[row as usize][col as usize] != ' ' {
                self.board[row as usize][col as usize] = ' ';
                break;
            }
        }
    }




    ///Returns the value of the game
    pub fn game_value(&mut self) -> i32 {
        if self.has_winner() == 'w' {
            match self.winner {
                Some('O') => i32::MAX,
                Some('T') => i32::MIN,
                _ => 0,
            }
        } else {
            0
        }
    }


    ///this will return a uszie representing the column the player wants to place
    ///Dont forget error handling
    /// allows_move() will be used to check if the move is valid
    pub fn get_player_move(&self) -> (u32, char) {
        loop {
            print!("Player's choice (enter as \"column token\"): ");
            io::stdout().flush().unwrap();
            let mut player_move = String::new();
            io::stdin().read_line(&mut player_move).unwrap();
            
            let player_move: Vec<&str> = player_move.trim().split_whitespace().collect();
            
            if player_move.len() != 2 {
                println!("Please enter a column and a token");
                continue;
            }

            let col: u32 = match player_move[0].parse() {
                Ok(col) => col,
                Err(_) => {
                    println!("Please enter a valid column number");
                    continue;
                }
            };

            let mut tok: char = match player_move[1].parse() {
                Ok(tok) => tok,
                Err(_) => {
                    println!("Please enter a valid token");
                    continue;
                }
            };

    
            if tok == 'o' {
                tok = 'O';
            } else if tok == 't' {
                tok = 'T';
            }

            if !(tok == 'O' || tok == 'T') {
                println!("Please enter a valid token");
                continue;
            }

            if !self.allows_move(col) {
                println!("Column is full, please choose another column");
                continue;
            }

            return (col, tok);

        }
    }

   //host the game on cli
    pub fn host_game(&mut self) -> String {
        println!("Welcome to Toot and Otto!\n");
        let mut game_over = false;
        let mut moves = String::new();
        let mut turn = 'O';
        while !game_over {
            println!("{}", self);

            if turn == 'O' {
                println!("Otto's turn");
            } else {
                println!("Toot's turn");
            }

            let (player_move_col, player_move_token) = self.get_player_move();
            self.perform_move_plz(player_move_col as u32, player_move_token, turn);
            moves.push_str(&player_move_col.to_string());

            //check if the game is over
            let winner = self.has_winner();
            if winner == 'w' {
                println!("{}", self);

                match self.winner {
                    Some('O') => println!("Otto wins -- Congratulations!"),
                    Some('T') => println!("Toot wins -- Congratulations!"),
                    _ => (),
                }
                game_over = true;
            } else if self.is_draw() {
                println!("{}", self);
                println!("It's a draw!");
                game_over = true;
            } else if winner == 't' {
                println!("{}", self);
                println!("Tie Game!");
                game_over = true;
            }
            turn = if turn == 'O' { 'T' } else { 'O' };
        }
        moves
    }

    //get the ai move for a state of the current board
    pub fn get_ai_move(&mut self, ai_tok:char, difficulty_var:u32) -> (u32, char){

        let difficulty = match difficulty_var {
            1 => Difficulty::Easy,
            2 => Difficulty::Hard,
            _ => Difficulty::Easy,
        };

        let ai = ottobot::OttoBot::new(difficulty, ai_tok);
        let ai_move_string = ai.best_move(self, ai_tok);
        let ai_move: Vec<char> = ai_move_string.chars().collect();
        let ai_move_char = ai_move[1];
        let ai_move = ai_move[0].to_digit(10).unwrap();
        (ai_move as u32, ai_move_char)
    }



    pub fn host_game_AI(&mut self, difficulty: Difficulty, tok: char) -> String {
        println!("Welcome to Toot and Otto!\n");
        let mut game_over = false;
        let mut moves = String::new();
        let mut ai_tok = if tok == 'T' { 'O' } else { 'T' };
        let ai = ottobot::OttoBot::new(difficulty, ai_tok);
        let mut turn = tok;
        while !game_over {
            println!("{}", self);
            
            let turn_name = if turn == 'O' { "Otto" } else { "Toot" };
            println!("{}'s turn", turn_name);

            if turn == ai_tok {
                println!("AI is thinking...");
                let ai_move_string = ai.best_move(self, ai_tok);
                let ai_move: Vec<char> = ai_move_string.chars().collect();
                let ai_move_char = ai_move[1];
                let ai_move = ai_move[0].to_digit(10).unwrap();
                self.perform_move_plz(ai_move as u32, ai_move_char, ai_tok);
                moves.push_str(&ai_move.to_string());
            } else {
                let (player_move_col, player_move_token) = self.get_player_move();
                self.perform_move_plz(player_move_col as u32, player_move_token, turn);
                moves.push_str(&player_move_col.to_string());
            }

            //check if the game is over
            let winner = self.has_winner();
            if winner == 'w' {
                println!("{}", self);

                match self.winner {
                    Some('O') => println!("Otto wins -- Congratulations!"),
                    Some('T') => println!("Toot wins -- Congratulations!"),
                    _ => (),
                }
                game_over = true;
            } else if self.is_draw() {
                println!("{}", self);
                println!("It's a draw!");
                game_over = true;
            } else if winner == 't' {
                println!("{}", self);
                println!("Tie Game!");
                game_over = true;
            }

            turn = if turn == 'O' { 'T' } else { 'O' };
        }
        moves
    }   
}

impl fmt::Display for TootOttoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.board {
            for &cell in row {
                write!(f, "{} ", cell)?;
            }
            writeln!(f)?;
        }
        for col in 0..self.width {
            write!(f, "{} ", col)?;
        }
        writeln!(f)?;
        Ok(())
    }
}