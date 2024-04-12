import * as games from "games";

class GameBoard {
    constructor(rows, cols, mode, game) {
        this.height = rows;
        this.width = cols;
        this.last_row = null;
        this.last_col = null;
        this.board = new Array(this.height).fill().map(() => new Array(this.width).fill(' '));
        this.last_player = null; // 0 for player 1, 1 for player 2
        this.mode = mode; // 0 for player vs player, 1 for easy AI, 2 for hard AI
        this.game = game;
        this.result = null; // 0 for player 1 win, 1 for player 2 win, 2 for draw
    }

    toggleLastPlayer() {
        if (this.last_player === null) {
            this.last_player = 0;
        } else {
            this.last_player = this.last_player === 0 ? 1 : 0;
        }
    }

    updateBoard(row, col, piece) {
        this.toggleLastPlayer();
        this.last_row = row;
        this.last_col = col;
        this.board[row][col] = piece;
    }

    checkWin() {
        // this.result = games.check_win(this.board, this.last_player, this.game, this.last_col, this.last_row);
        // console.log(this.result);
        // return this.result != null
        try {
            const result = games.check_win(this.board, this.last_player, this.game, this.last_col, this.last_row);
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

export function drawBoard(size, mode, gameName) {
    var rows, cols, gameBoard
    if (gameName == "connect4") {
        gameBoard = document.getElementById('connect4GameBoard');
        rows = size == 0 ? 6 : 7;
        cols = size == 0 ? 7 : 10;
    } else {
        gameBoard = document.getElementById('TootOttoGameBoard');
        rows = size == 0 ? 4 : 6;
        cols = size == 0 ? 6 : 9;
    }
    gameBoard.innerHTML = ''; 
    var table = document.createElement('table');
    table.className = "ui";
    let game = new GameBoard(rows, cols, mode, gameName);

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
                if (game.result === null) {
                    getPlayerMove(this.id, game);
                }
            };
            cell.appendChild(input);
            row.appendChild(cell);
        }
        table.appendChild(row);
    }
    gameBoard.appendChild(table);
}


function getPlayerMove(cell_selected, game) {
    var maxRows = game.height;
    var maxCols = game.width;
    var selectedColumn = parseInt(cell_selected.substring(1), 10) % maxCols;
    var cellId = getEmptyCell(selectedColumn, maxRows, maxCols);

    if (cellId == -1) {
        return;
    }

    performMove(cellId, game);

    // AI move
    if (!endGame(game) && game.mode != 0) {
        getAIMove(game);
    }
}

function endGame(game) {
    if (!game.checkWin()) {
        return false;
    }
    if (game.result == 2) {
        alert("Draw!");
    } else if (game.result == 0) {
        alert("Player 1 has won!");
    } else {
        alert("Player 2 has won!");
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
    if (game.game == "connect4") {
        if (game.last_player === 0) {
            cell.classList.add('yellow-filled') 
            piece = 'X';
        } else {
            cell.classList.add('red-filled');
            piece = 'O';
        }

    } else {
        var tokens = document.getElementsByName('token');
        for (var i = 0; i < tokens.length; i++) {
            if (tokens[i].checked) {
                piece = tokens[i].value;
                break;
            }
        }
        if (piece == 'T') {
            cell.classList.add('toot-token');
            cell.value = piece;
        } else {
            cell.classList.add('otto-token');
            cell.value = piece;
        }
    }
    
    let row = Math.floor(parseInt(cellId.substring(1), 10) / game.width);
    let col = parseInt(cellId.substring(1), 10) % game.width;
    
    game.updateBoard(row, col, piece);
}