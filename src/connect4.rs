use std::cmp;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::fmt;
use super::board::*;

pub struct Connect4Board {
    width: usize,
    height: usize,
    board: Vec<Vec<char>>,
    last_row: Option<usize>,
    last_col: Option<usize>,
    last_player: Option<char>,
}

impl Connect4Board {
    pub fn new(size: BoardSize) -> Connect4Board {
        let (width, height) = match size {
            BoardSize::Standard => (7, 6),
            BoardSize::Large => (10, 7),
        };

        Connect4Board {
            width,
            height,
            board: vec![vec![' '; width]; height],
            last_row: None,
            last_col: None,
            last_player: None,
        }
    }

    pub fn available_moves(&self) -> Vec<usize> {
        (0..self.width).filter(|&col| self.allows_move(col)).collect()
    }

    pub fn allows_move(&self, col: usize) -> bool {
        col < self.width && self.board[0][col] == ' '
    }

    pub fn perform_move(&mut self, col: usize, ox: char) {
        for row in (0..self.height).rev() {
            if self.board[row][col] == ' ' {
                self.board[row][col] = ox;
                self.last_row = Some(row);
                self.last_col = Some(col);
                self.last_player = Some(ox);
                break;
            }
        }
    }

    pub fn undo_move(&mut self, col: usize) {
        for row in 0..self.height {
            if self.board[row][col] != ' ' {
                self.board[row][col] = ' ';
                break;
            }
        }
    }

    pub fn is_terminal(&self) -> bool {
        self.has_winner() || self.is_draw()
    }

    pub fn has_winner(&self) -> bool {
        let row = self.last_row;
        let col = self.last_col;
        let ox = self.last_player;

        if row.is_none() || col.is_none() || ox.is_none() {
            return false;
        }

        let row = row.unwrap();
        let col = col.unwrap();
        let ox = ox.unwrap();

        for c in cmp::max(0, col as i32 - 3) as usize..cmp::min(self.width - 3, col + 1) {
            if [self.board[row][c], self.board[row][c + 1], self.board[row][c + 2], self.board[row][c + 3]].iter().all(|&x| x == ox) {
                return true;
            }
        }

        if row < self.height - 3 {
            if [self.board[row][col], self.board[row + 1][col], self.board[row + 2][col], self.board[row + 3][col]].iter().all(|&x| x == ox) {
                return true;
            }
        }

        for i in 0..4 {
            let r = row as i32 - i as i32;
            let c = col as i32 - i as i32;
            if r >= 0 && r < (self.height - 3) as i32 && c >= 0 && c < (self.width - 3) as i32 {
                let r = r as usize;
                let c = c as usize;
                if [self.board[r][c], self.board[r + 1][c + 1], self.board[r + 2][c + 2], self.board[r + 3][c + 3]].iter().all(|&x| x == ox) {
                    return true;
                }
            }
        }

        for i in 0..4 {
            let r = row as i32 - i as i32;
            let c = col as i32 + i as i32;
            if r >= 0 && r < (self.height - 3) as i32 && c >= 3 && c < self.width as i32 {
                let r = r as usize;
                let c = c as usize;
                if [self.board[r][c], self.board[r + 1][c - 1], self.board[r + 2][c - 2], self.board[r + 3][c - 3]].iter().all(|&x| x == ox) {
                    return true;
                }
            }
        }

        false
    }

    pub fn is_draw(&self) -> bool {
        self.available_moves().is_empty()
    }

    pub fn game_value(&self) -> i32 {
        if self.has_winner() {
            match self.last_player {
                Some('X') => i32::MAX,
                Some('O') => i32::MIN,
                _ => 0,
            }
        } else {
            0
        }
    }

    pub fn get_player_move(&mut self, ox: char) -> usize {
        let ai = Connect4AI::new(Difficulty::Hard);
        loop {
            print!("{}'s choice: ", ox);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            match input.parse::<usize>() {
                Ok(col_move) => {
                    if self.allows_move(col_move) {
                        return col_move;
                    }
                }
                Err(_) => continue,
            }
        }
    }

    pub fn print_congrats(&self) {
        if let Some(last_player) = self.last_player {
            println!("{}", self);
            println!("{} wins -- Congratulations!", last_player);
        }
    }

    pub fn host_game(&mut self) -> String {
        println!("Welcome to Connect Four!\n");
        let mut game_over = false;
        let mut moves = String::new();
        let mut ox = 'X';
        while !game_over {
            println!("{}", self);
            let col_move = self.get_player_move(ox);
            self.perform_move(col_move, ox);
            moves.push_str(&col_move.to_string());
            if self.has_winner() {
                game_over = true;
            }
            ox = if ox == 'X' { 'O' } else { 'X' };
        }
        self.print_congrats();
        moves
    
    }

    pub fn host_game_AI(&mut self, difficulty: Difficulty) -> String {
        println!("Welcome to Connect Four vs AI mode!\n");
        let mut game_over = false;
        let mut moves = String::new();
        let mut ox = 'X';
        let ai = Connect4AI::new(difficulty);
        while !game_over {
            println!("{}", self);
            let col_move = {
                if ox == 'O' {
                    ai.best_move(self, ox)
                } else {
                    self.get_player_move(ox)
                }
            };
            self.perform_move(col_move, ox);
            moves.push_str(&col_move.to_string());
            if self.has_winner() {
                game_over = true;
            }
            ox = if ox == 'X' { 'O' } else { 'X' };
        }
        self.print_congrats();
        moves
    
    }

}

impl fmt::Display for Connect4Board {
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

pub struct Connect4AI {
    depth: u32,
}

impl Connect4AI {
    pub fn new(difficulty: Difficulty) -> Connect4AI {
        let depth = match difficulty {
            Difficulty::Easy => 1,
            Difficulty::Hard => 5,
        };
        Connect4AI { depth }
    }

    fn best_move(&self, board: &mut Connect4Board, ox: char) -> usize {
        let maximizing_player = ox == 'X';
        self.minimax(board, self.depth, i32::MIN, i32::MAX, maximizing_player).1
    }
    fn evaluate_window(&self, window: &[char], player: char) -> i32 {
        let mut score = 0;
        let opponent = if player == 'X' { 'O' } else { 'X' };

        let player_count = window.iter().filter(|&&ox| ox == player).count();
        let opponent_count = window.iter().filter(|&&ox| ox == opponent).count();
        let empty_count = window.iter().filter(|&&ox| ox == ' ').count();

        if player_count == 4 {
            score += 100;
        } else if player_count == 3 && empty_count == 1 {
            score += 5;
        } else if player_count == 2 && empty_count == 2 {
            score += 2;
        }

        if opponent_count == 3 && empty_count == 1 {
            score -= 4;
        }

        score
    }

    fn score_position(&self, board: &Connect4Board, maximizing_player: bool) -> i32 {
        let mut score = 0;
        let player = if maximizing_player { 'X' } else { 'O' };

        // Score center column: the more pieces in the center, the better
        let center_array: Vec<char> = board.board.iter().map(|row| row[board.width / 2]).collect();
        let center_count = center_array.iter().filter(|&&ox| ox == player).count() as i32;
        score += center_count * 3;

        // Score horizontal: the more pieces in a row, the better
        for row in &board.board {
            for window in row.windows(4) {
                score += self.evaluate_window(window, player);
            }
        }

        // Score vertical: the more pieces in a column, the better
        for col in 0..board.width {
            let col_array: Vec<char> = board.board.iter().map(|row| row[col]).collect();
            for window in col_array.windows(4) {
                score += self.evaluate_window(window, player);
            }
        }

        // Score positive sloped diagonal: the more pieces in a diagonal, the better
        for row in 0..(board.height - 3) {
            for col in 0..(board.width - 3) {
                let window: Vec<char> = (0..4).map(|i| board.board[row + i][col + i]).collect();
                score += self.evaluate_window(&window, player);
            }
        }

        // Score negative sloped diagonal: the more pieces in a diagonal, the better
        for row in 3..board.height {
            for col in 0..(board.width - 3) {
                let window: Vec<char> = (0..4).map(|i| board.board[row - i][col + i]).collect();
                score += self.evaluate_window(&window, player);
            }
        }
        score
    }


    fn minimax(&self, board: &mut Connect4Board, depth: u32, alpha: i32, beta: i32, maximizing_player: bool) -> (i32, usize) {
        let mut best_score = if maximizing_player { i32::MIN } else { i32::MAX };
        let mut alpha = alpha;
        let mut beta = beta;
        if depth == 0 || board.is_terminal() {
            best_score = { 
                if board.is_terminal() { // win, lose, or draw
                    board.game_value()
                } else { // evaluate the odds of player winning in this position
                    self.score_position(board, maximizing_player) 
                }
            };
            return (best_score, 0)
        }

        let mut best_move = board.available_moves().choose(&mut rand::thread_rng()).unwrap().clone();

        for &i in board.available_moves().iter() {
            board.perform_move(i, if maximizing_player { 'X' } else { 'O' });
            let score = self.minimax(board, depth - 1, alpha, beta, !maximizing_player).0;
            board.undo_move(i);
            if maximizing_player {
                if score > best_score {
                    best_move = i;
                    best_score = score;
                }
                alpha = cmp::max(alpha, best_score);
                if alpha >= beta {
                    break;
                }
            } else {
                if score < best_score {
                    best_move = i;
                    best_score = score;
                }
                beta = cmp::min(beta, best_score);
                if alpha >= beta {
                    break;
                }

            }
        }
        
        (best_score, best_move.clone())
    }
}