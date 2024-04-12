import * as games from "connect4";

class GameBoard {
    constructor(rows, cols, mode) {
        this.height = rows;
        this.width = cols;
        this.board = new Array(this.height).fill().map(() => new Array(this.width).fill(' '));
        this.mode = mode; // 0 for player vs player, 1 for easy AI, 2 for hard AI
        this.winner = null; // 0 for O win, 1 for T win, 2 for draw, 3 for tie
        this.turn = 'T'; // T for Toot, O for Otto
    }

    init(size){
        games.toot_init_board(size)
    }

    updateBoard(row, col, piece) {
        this.board[row][col] = piece;
    }

    checkWin() {
        // this.result = games.check_win(this.board, this.last_player, this.game, this.last_col, this.last_row);
        // console.log(this.result);
        // return this.result != null
        try {
            const result = games.toot_has_winner();
            // this.result = games.check_win(this);
            console.log(result);  
            return this.result !== null;
        } catch (error) {
            console.error("Failed to process game data:", error);
            return false;
        }
    }


    bestMove() {
        try {
            let move = games.best_move(this);
            console.log(move);  
            return move;
        } catch (error) {
            console.error("Failed to process game data:", error);
            return 0;
        }
        
        // return games.best_move(this.board, this.last_player, this.game, this.last_col, this.last_row, this.mode);
    }

}

export function drawBoardToot(size, mode) {
    var rows, cols, gameBoard

    gameBoard = document.getElementById('TootOttoGameBoard');
    rows = size == 0 ? 4 : 6;
    cols = size == 0 ? 6 : 9;

    gameBoard.innerHTML = ''; 
    var table = document.createElement('table');
    table.className = "ui";

    let game = new GameBoard(rows, cols, mode);
    game.init(size);

    console.log("drawBoardToot called");

    for (var i = 0; i < rows; i++) {
        var row = document.createElement('tr');
        row.className = "row";
        for (var j = 0; j < cols; j++) {
            var cell = document.createElement('td');
            var input = document.createElement('input');
            input.type = "text";
            input.id = "b" + (i * cols + j);
            input.className = ["cell", "empty-cell"].join(' ');
            input.readOnly = true;
            input.onclick = function() {
                if (game.toot_is_terminal() === false) {
                    getMove(this.id, game);
                }
            };
            cell.appendChild(input);
            row.appendChild(cell);
        }
        table.appendChild(row);
    }
    gameBoard.appendChild(table);
}


function getMove(cell_selected, game) {
    var maxRows = game.height;
    var maxCols = game.width;
    var selectedColumn = parseInt(cell_selected.substring(1), 10) % maxCols;
    var cellId = getEmptyCell(selectedColumn, maxRows, maxCols);

    if (cellId == -1 || game.toot_allows_move(selectedColumn) === false){
        return;
    }

    performMove(cellId, game);

    // AI move
    if (!endGame(game) && game.mode != 0) {
        getAIMove(game);
    }
}

function endGame(game) {
    if (!game.toot_is_terminal) {
        return false;
    }

    let result = game.toot_has_winner();
    if (result === 'w') {
        //there is a winner, we should get winner
        let winner = game.toot_get_winner();
        if (winner === 'T') {
            alert("TOOT HAS WON!");
        } else {
            alert("OTTO HAS WON!");
        }
    }


    return true;
}

function getAIMove(game) {
    var maxRows = game.height;
    var maxCols = game.width;
    var selectedColumn = game.bestMove();
    var cellId = getEmptyCell(selectedColumn, maxRows, maxCols);

    if (cellId == -1) {
        return;
    }
    
    performMove(cellId, game);
    
    var bool = endGame(game);
}

function getEmptyCell(selectedColumn, maxRows, maxCols) {
    for (var i = maxRows - 1; i >= 0; i--) {
        var cellId = 'b' + (i * maxCols + selectedColumn);
        var cell = document.getElementById(cellId);
        if (cell.classList.contains('empty-cell')) {
            return cellId;
        }
    }
    return -1;
}

function performMove(cellId, game) {
    let cell = document.getElementById(cellId);
    cell.classList.remove('empty-cell');
    var piece;
    var tokens = document.getElementsByName('token');
    for (var i = 0; i < tokens.length; i++) {
        if (tokens[i].checked) {
            piece = tokens[i].value;
            break;
        }
    }
        
    let row = Math.floor(parseInt(cellId.substring(1), 10) / game.width);
    let col = parseInt(cellId.substring(1), 10) % game.width;

    if (piece == 'T') {
        cell.classList.add('toot-token');
        cell.value = piece;

        game.toot_perform_move(col, 'T', game.turn);
    } else {
        cell.classList.add('otto-token');
        cell.value = piece;
        game.toot_perform_move(col, 'O', game.turn);
    }

    game.turn = game.turn == 'T' ? 'O' : 'T';
    game.updateBoard(row, col, piece);
}
