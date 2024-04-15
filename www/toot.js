import { BoardSize,Difficulty,TootOttoBoard,OttoBot,deserialize_otto } from "connect4";

class GameBoard {
    constructor(size, mode, playerTok, savedGame) {
        if (savedGame != undefined) {
            this.size = savedGame.size;
            this.mode = savedGame.mode;
            this.turn = savedGame.turn;
            this.winner = savedGame.winner; // 0: player 1, 1: player 2, 2: draw
            this.playerTok = savedGame.playerTok;
            this.ai = this.mode !== null ? new OttoBot(this.get_mode(this.mode), this.playerTok == 'T' ? 'O' : 'T') : null;
        }
        else {
            this.size = size == 0 ? BoardSize.Standard : BoardSize.Large;
            this.board = new TootOttoBoard(this.size);
            this.mode = mode; // 0 for player vs player, 1 for easy AI, 2 for hard AI
            this.winner = null; // 0 for O win, 1 for T win, 2 for draw, 3 for tie
            this.turn = 'T'; // T for Toot, O for Otto
            this.playerTok = playerTok;
            this.ai = this.mode !== null ? new OttoBot(this.get_mode(this.mode), this.playerTok == 'T' ? 'O' : 'T') : null;
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
        this.turn = this.turn === 'T' ? 'O' : 'T';
    }

    updateBoard(row, col, piece) {
        this.board[row][col] = piece;
    }

}

export function newGameToot(size, mode, playerTok) {
    console.log("creating new game")
    var game = new GameBoard(size, mode, playerTok);
    saveGame(game);

    // initialize message
    let winnerDisplay = document.getElementById("winnerDisplay");
    winnerDisplay.innerHTML = "Game in progress...";
};


export function drawBoardToot(size, mode, playerTok) {
    var rows, cols, gameBoard, game;

    game = loadGame();
    if (game == null) {
        // couldnt load, create new
        newGameToot(size, mode, playerTok);
        game = loadGame();        
    }
    
    gameBoard = document.getElementById('TootOttoGameBoard');
    rows = game.size == BoardSize.Standard ? 4 : 6;
    cols = game.size == BoardSize.Standard ? 6 : 9;
    gameBoard.innerHTML = ''; 
    var table = document.createElement('table');
    table.className = "ui";

    game.turn = playerTok;

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
                if (game.winner === null) {
                    startTurn(this.id, game);
                }
            };

            // colour input
            var colour = game.board.get_colour(i, j);
            if (colour == 'T') {
                input.classList.add('toot-token');
            } else if (colour == 'O') {
                input.classList.add('otto-token');
            }

            cell.appendChild(input);
            row.appendChild(cell);
        }
        table.appendChild(row);
    }
    gameBoard.appendChild(table);

    setFormFieldsToot(game);
}


function startTurn(cell_selected, game) {
    var maxRows = game.board.height();
    var maxCols = game.board.width();
    var selectedColumn = parseInt(cell_selected.substring(1), 10) % maxCols;
    var cellId = getEmptyCell(selectedColumn, maxRows, maxCols);
    console.log("Player move: " + cellId);

    if (cellId == -1 || game.board.allows_move(selectedColumn) === false){
        return;
    }

    performMove(cellId, game);

    // AI move
    let end = endGame(game, false);
    if (!end && game.mode != 0) {
        getAIMove(game);
    }
    // if game over, alert
    else if (end) {
        endGame(game, true);
    }
}

function endGame(game, doAlert) {
    var msg;
    let winnerDisplay = document.getElementById("winnerDisplay");
    let result = game.board.has_winner();

    if (!game.board.is_terminal()) {
        winnerDisplay.innerHTML = "Game in progress...";
        return false;
    }

    if (result === 'w') {
        //there is a winner, we should get winner
        let winner = game.board.get_winner();
        if (winner === 'T') {
            game.winner = 1;
            msg = "TOOT has won!"
        } else {
            game.winner = 0;
            msg = "OTTO has won!"
        }
    }else if (result === 'd') {
        game.winner = 2;
        msg = "It's a draw!"
    }else if (result === 't') {
        game.winner = 3;
        msg = "It's a tie!"
    }

    winnerDisplay.innerHTML = msg
    if (doAlert == true) {
        alert(msg);
    }
    return true;
}

function getAIMove(game) {
    console.log("AI move");
    var maxRows = game.board.height();
    var maxCols = game.board.width();
    var ai_token = game.playerTok == 'T' ? 'O' : 'T';
    var selectedColumnandtoken= game.ai.best_move(game.board, ai_token);
    var selectedColumn = parseInt(selectedColumnandtoken.substring(0,1), 10);
    var token = selectedColumnandtoken.substring(1,2);
    var cellId = getEmptyCell(selectedColumn, maxRows, maxCols);

    if (cellId == -1) {
        return;
    }
    
    performMoveAI(cellId,token, game);
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
        
    let col = parseInt(cellId.substring(1), 10) % game.board.width();

    if (piece == 'T') {
        cell.classList.add('toot-token');
        cell.value = piece;
    } else {
        cell.classList.add('otto-token');
        cell.value = piece;
    }

    var turn = game.turn == 'T' ? 'T' : 'O';
    game.board.perform_move_plz(col, piece, turn);
    game.nextTurn();

    saveGame(game);
}

function performMoveAI(cellId, ai_token, game) {
    //funciton to hanlde AI move
    let cell = document.getElementById(cellId);
    cell.classList.remove('empty-cell');
    var piece = ai_token;
        
    let col = parseInt(cellId.substring(1), 10) % game.board.width();

    if (piece == 'T') {
        cell.classList.add('toot-token');
        // cell.value = piece;
    } else {
        cell.classList.add('otto-token');
        // cell.value = piece;
    }

    var turn = game.turn == 'O' ? 'O' : 'T';
    console.log("AI Turn " + turn)
    game.board.perform_move_plz(col, piece, turn);
    game.nextTurn();

    saveGame(game);
}


function saveGame(game) {
    console.log("saveGame called");
    console.log(game);
    
    // serialize this thing
    var sg = JSON.stringify(game);
    console.log(sg);
    localStorage.setItem("otto", sg);

    // serialize gameBoard
    var sb = game.board.serialize();
    console.log(sb);
    localStorage.setItem("ottoboard", sb);
}

function loadGame() {
    console.log("loadGame called");
    var game, board, reconst;
    var storedGameJSON = localStorage.getItem("otto");
    var storedBoardJSON = localStorage.getItem("ottoboard");

    if (storedGameJSON == null || storedBoardJSON == null) {
        console.log("no stored game");
        return null;
    }

    console.log("stored game: " + storedGameJSON);
    game = JSON.parse(storedGameJSON);

    console.log("stored board: " + storedBoardJSON);
    board = deserialize_otto(storedBoardJSON);

    reconst = new GameBoard(null, null, null, game);
    reconst.board = board;
    console.log(reconst);

    return reconst;
}

export function onLoadToot() {
    console.log("onLoadToot");
    var storedGameJSON = localStorage.getItem("otto");
    var storedBoardJSON = localStorage.getItem("ottoboard");

    if (storedGameJSON == null || storedBoardJSON == null) {
        console.log("no stored game on load");
    }
    else {
        drawBoardToot();
    }
};

function setFormFieldsToot(game) {
    let boardSizeRadio = document.getElementById("board_" + game.size);
    boardSizeRadio.checked = true;

    var mode;
    if (game.mode == null) {
        mode = "0";
    } else if (game.mode == Difficulty.Easy) {
        mode = "1";
    } else {
        mode = "2";
    }
    
    let gameModeRadio = document.getElementById("gamemode_" + mode);
    gameModeRadio.checked = true;

    let playerTokRadio = document.getElementById("player_" + game.playerTok);
    playerTokRadio.checked = true;
};