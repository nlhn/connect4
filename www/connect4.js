import { BoardSize, Difficulty, Connect4Board, Connect4AI, deserialize_connect4 } from "connect4";
import { handleTheme } from "./theme";

class GameData {
    constructor(size, mode, savedGame) {
        if (savedGame != undefined) {
            this.size = savedGame.size;
            this.mode = savedGame.mode;
            this.turn = savedGame.turn;
            this.winner = savedGame.winner; // 0: player 1, 1: player 2, 2: draw
            this.backendBoard = null;
            this.ai = this.mode !== null ? new Connect4AI(this.mode) : null;
        }
        else {
            this.size = size == 0 ? BoardSize.Standard : BoardSize.Large;
            this.mode = this.get_mode(mode);
            this.turn = 'X';
            this.winner = null; // 0: player 1, 1: player 2, 2: draw
            this.backendBoard = new Connect4Board(this.size);
            this.ai = this.mode !== null ? new Connect4AI(this.mode) : null;
        }
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

export function newGame(size, mode) {
    console.log("creating new game")
    var game = new GameData(size, mode);
    saveGame(game);

    // initialize message
    let winnerDisplay = document.getElementById("winnerDisplay");
    winnerDisplay.innerHTML = "Game in progress...";
};

export function drawBoard(size, mode) {
    var rows, cols, gameBoard, game;
    
    game = loadGame();
    if (game == null) {
        // couldnt load, create new
        newGame(size, mode);
        game = loadGame();        
    }

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

            // colour input
            var colour = game.backendBoard.get_colour(i, j);
            if (colour == 'X') {
                input.classList.add('yellow-filled');
            } else if (colour == 'O') {
                input.classList.add('red-filled');
            }

            cell.appendChild(input);
            row.appendChild(cell);
        }
        table.appendChild(row);
    }
    gameBoard.appendChild(table);
    endGame(game, false);
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
    if (!endGame(game, false) && game.ai != null) {
        getAIMove(game);
        endGame(game, true);
    }
}

function endGame(game, doAlert) {
    var msg;
    let winnerDisplay = document.getElementById("winnerDisplay");

    if (!game.backendBoard.is_terminal()) {
        msg = "Game in progress...";
        winnerDisplay.innerHTML = msg
        return false;
    }
    else {
        let winner = game.getWinner();
        if (winner == 2) {
            msg = "Draw!"
        } else if (winner == 0) {
            msg = "Player 1 has won!";
        } else {
            msg = "Player 2 has won!";
        }
        winnerDisplay.innerHTML = msg
        if (doAlert == true) {
            alert(msg);
        }
        return true;
    }
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

    saveGame(game);
    handleTheme(); // token changed
}

function saveGame(game) {
    console.log("saveGame called");
    
    // serialize this thing
    var sg = JSON.stringify(game);
    console.log(sg);
    localStorage.setItem("connect4", sg);

    // serialize gameBoard
    var sb = game.backendBoard.serialize();
    console.log(sb);
    localStorage.setItem("connect4board", sb);
}

function loadGame() {
    console.log("loadGame called");
    var game, board, reconst;
    var storedGameJSON = localStorage.getItem("connect4");
    var storedBoardJSON = localStorage.getItem("connect4board");

    if (storedGameJSON == null || storedBoardJSON == null) {
        console.log("no stored game");
        return null;
    }

    console.log("stored game: " + storedGameJSON);
    game = JSON.parse(storedGameJSON);

    console.log("stored board: " + storedBoardJSON);
    board = deserialize_connect4(storedBoardJSON);

    reconst = new GameData(null, null, game);
    reconst.backendBoard = board;
    console.log(reconst);

    return reconst;
}

export function onLoad() {
    var storedGameJSON = localStorage.getItem("connect4");
    var storedBoardJSON = localStorage.getItem("connect4board");

    if (storedGameJSON == null || storedBoardJSON == null) {
        console.log("no stored game on load");
    }
    else {
        drawBoard();
    }
};
