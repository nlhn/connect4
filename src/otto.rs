use std::{collections:: HashSet, fmt};
use super::board::*;
use std::io::{self, Write};
use crate::ottobot;

///Player interacts directly with the board
///and the board interacts with the bot where the bot will
///"return" a move to place on the board
///
/// The board will keep track of the game state and the bot 

pub struct TootOttoBoard {
    pub board: Vec<Vec<char>>,

    //for custom size
    pub width: usize,
    pub height: usize,

    //winning cond checkers
    pub last_player: Option<char>, //'o' for otto and 't' for toot
    pub last_row: Option<usize>,
    pub last_col: Option<usize>,

    pub winner: Option<char>,
}

impl TootOttoBoard {
    pub fn new(size: BoardSize) -> TootOttoBoard {
        let (width, height) = match size {
            BoardSize::Standard => (6, 4),
            BoardSize::Large => (9, 6),
        };

        TootOttoBoard {
            width,
            height,
            board: vec![vec![' '; width]; height],
            last_row: None,
            last_col: None,
            last_player: None, 
            winner: None,
        }

    }

    /// Returns a list(HashSet) of available moves on the board
    pub fn available_moves(&self) -> Vec<usize>{
        let mut moves = Vec::new();
        for col in 0..self.width {
            if self.allows_move(col) {
                moves.push(col);
            }
        }
        return moves;
    }

    ///Takes a column and reutrns true if a move can be made into
    ///that column. False otherwise
    pub fn allows_move(&self, col: usize) -> bool{
        col < self.width && self.board[0][col] == ' '
    }

    ///Takes a column and a token and places the token on that column
    pub fn perform_move(&mut self, col: usize, tok: char, player: char) {
        /// decrement from the bottom row to the top row
        for row in (0..self.height).rev(){
            if self.board[row][col] == ' ' {
                self.board[row][col] = tok;
                self.last_row = Some(row);
                self.last_col = Some(col);
                self.last_player = Some(player);
                break;
            }
        }
    }


    ///Takes a row and column and returns the token at that position
    pub fn get(&self, row: usize, col: usize) -> char {
        self.board[row][col]
    }

    ///Takes a column and removes the token from that column
    /// this is used to undo a move (for AI)
    pub fn undo_move(&mut self, col: usize) {
        for row in 0..self.height {
            if self.board[row][col] != ' ' {
                self.board[row][col] = ' ';
                break;
            }
        }
    }

    pub fn is_terminal(&mut self) -> bool {
        if (self.has_winner() !='f') || self.is_draw() {
            return true;
        }
        return false;
    }

    ///Takes a board and checks if the board is in a winning position
    ///returns "w" if there is a winner
    /// "t" if the game is a tie
    /// "f" if the game is still in progress
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
                let token = self.board[row][col];

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
                    && (self.board[row][col + 1] == opposite)
                    && (self.board[row][col + 2] == opposite)
                    && (self.board[row][col + 3] == token)
                {
                    winners_set.insert(token);
                    continue;
                }

                if (row as i32) - 3 >= 0 {
                    // Check up
                    if(self.board[row-1][col] == opposite)
                        &&(self.board[row-2][col] == opposite)
                        &&(self.board[row-3][col] == token)
                    {
                        winners_set.insert(token);
                        continue;
                    }

                    // Check up and right
                    if col  + 3 < cols
                        && (self.board[row-1][col+1] == opposite)
                        && (self.board[row-2][col+2] == opposite)
                        && (self.board[row-3][col+3] == token)
                    {
                        winners_set.insert(token);
                        continue;
                    }

                    // Check up and left
                    if (col as i32) - 3 >= 0
                        && (self.board[row-1][col-1] == opposite)
                        && (self.board[row-2][col-2] == opposite)
                        && (self.board[row-3][col-3] == token)
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

    pub fn is_draw(&self) -> bool {
        self.available_moves().is_empty()
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
    pub fn get_player_move(&self) -> (usize, char) {
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

            let col: usize = match player_move[0].parse() {
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
            self.perform_move(player_move_col, player_move_token, turn);
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
                let (ai_move, ai_move_char) = ai.best_move(self, ai_tok);
                self.perform_move(ai_move, ai_move_char, ai_tok);
                moves.push_str(&ai_move.to_string());
            } else {
                let (player_move_col, player_move_token) = self.get_player_move();
                self.perform_move(player_move_col, player_move_token, turn);
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