import { BoardSize, Difficulty, Connect4Board, Connect4AI } from "connect4";

class GameData {
    constructor(size, mode) {
        this.size = size == 0 ? BoardSize.Standard : BoardSize.Large;
        this.mode = this.get_mode(mode);
        this.turn = 'X';
        this.winner = null; // 0: player 1, 1: player 2, 2: draw
        this.backendBoard = new Connect4Board(this.size);
        this.ai = this.mode !== null ? new Connect4AI(this.mode) : null;
    }

    get_mode(mode) {
        if (mode == 0) {
            return null;
        } else if (mode == 1) {
            return Difficulty.Easy;
        } else {
            return Difficulty.Hard;
        }
    }

    nextTurn() {
        this.turn = this.turn === 'X' ? 'O' : 'X';
    }

    getWinner() {
        if (this.backendBoard.is_draw()) {
            this.winner = 2;
        } else if (this.backendBoard.last_player() == 'X') {
            this.winner = 0;
        } else {
            this.winner = 1;
        }
        return this.winner;
    }


}

export function drawBoard(size, mode, gameName) {
    var rows, cols, gameBoard;
    var game = new GameData(size, mode);
    gameBoard = document.getElementById('connect4GameBoard');
    rows = game.size == BoardSize.Standard ? 6 : 7;
    cols = game.size == BoardSize.Standard ? 7 : 10;

    gameBoard.innerHTML = ''; 
    var table = document.createElement('table');
    table.className = "ui";

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
                if (game.winner === null) {
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
    var maxRows = game.backendBoard.height();
    var maxCols = game.backendBoard.width();
    var selectedColumn = parseInt(cell_selected.substring(1), 10) % maxCols;
    var cellId = getEmptyCell(selectedColumn, maxRows, maxCols);

    if (cellId == -1) {
        return;
    }

    performMove(cellId, game);

    // AI move
    if (!endGame(game) && game.ai != null) {
        getAIMove(game);
    }
}

function endGame(game) {
    if (!game.backendBoard.is_terminal()) {
        return false;
    }
    let winner = game.getWinner();
    if (winner == 2) {
        alert("Draw!");
    } else if (winner == 0) {
        alert("Player 1 has won!");
    } else {
        alert("Player 2 has won!");
    }
    return true;
}

function getAIMove(game) {
    var maxRows = game.backendBoard.height();
    var maxCols = game.backendBoard.width();
    var selectedColumn = game.ai.best_move(game.backendBoard, 'O');
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
    console.log("cellId: ", cellId);
    cell.classList.remove('empty-cell');
    var piece;
    if (game.turn == 'X') {
        cell.classList.add('yellow-filled') 
        piece = 'X';
    } else {
        cell.classList.add('red-filled');
        piece = 'O';
    }
    let col = parseInt(cellId.substring(1), 10) % game.backendBoard.width();
    game.backendBoard.perform_move(col, piece);
    game.nextTurn();
}