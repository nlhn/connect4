import * as games from "connect4";

class GameBoard {
    constructor(rows, cols) {
        this.height = rows;
        this.width = cols;
        this.last_row = null;
        this.last_col = null;
        this.board = new Array(rows).fill(null).map(() => new Array(cols).fill(null));
        this.last_player = null; // 0 for player 1, 1 for player 2
    }
    toggleLastPlayer() {
        if (this.last_player === null) {
            this.last_player = 0;
        } else {
            this.last_player = this.last_player === 0 ? 1 : 0;
        }
    }

    // performMove(row, col) {

    // }

    // hasWon() {
    //     return games.checkWin(this.board, this.last_player);
    // }

    // bestMove() {
    //     return games.bestMove(this.board, this.last_player);
    // }
}

document.getElementById('Connect4Button').addEventListener('click', function() {
    var boardSizeElements = document.getElementsByName('boardSize');
    var gameModeElements = document.getElementsByName('gameMode');
    var selectedSize = -1;
    var selectedMode = -1;

    for (var i = 0; i < boardSizeElements.length; i++) {
        if (boardSizeElements[i].checked) {
            selectedSize = parseInt(boardSizeElements[i].value, 10);
            console.log('Selected board size:', selectedSize);
            break;
        }
    }

    for (var i = 0; i < gameModeElements.length; i++) {
        if (gameModeElements[i].checked) {
            selectedMode = parseInt(gameModeElements[i].value, 10);
            console.log('Selected game mode:', selectedMode);
            break;
        }
    }
    if (selectedSize != -1 && selectedMode != -1) {
        drawBoard(selectedSize, selectedMode);
    }

});

function drawBoard(size, mode) {
    var gameBoard = document.getElementById('connect4GameBoard');
    gameBoard.innerHTML = ''; 
    var table = document.createElement('table');
    table.className = "ui";
    const rows = size == 0 ? 6 : 7;
    const cols = size == 0 ? 7 : 10;

    let game = new GameBoard(rows, cols);

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
                getPlayerMove(this.id, game);
            };
            cell.appendChild(input);
            row.appendChild(cell);
        }
        table.appendChild(row);
    }
    gameBoard.appendChild(table);
}


function getPlayerMove(id, game) {
    var maxRows = game.height;
    var maxCols = game.width;
    var selectedColumn = parseInt(id.substring(1), 10) % maxCols;

    for (var i = maxRows - 1; i >= 0; i--) {
        var cellId = 'b' + (i * maxCols + selectedColumn);
        var cell = document.getElementById(cellId);
        if (cell.classList.contains('empty-cell')) {
            game.toggleLastPlayer();
            cell.classList.remove('empty-cell');
            if (game.last_player === 0) {
                cell.classList.add('yellow-filled');
            } else {
                cell.classList.add('red-filled');
            }
            break;
        }
    }
}
