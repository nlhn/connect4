import { BoardSize,Difficulty,TootOttoBoard,OttoBot } from "connect4";

class GameBoard {
    constructor(size, mode, playerTok) {
        this.size = size == 0 ? BoardSize.Standard : BoardSize.Large;
        this.board = new TootOttoBoard(this.size);
        this.mode = mode; // 0 for player vs player, 1 for easy AI, 2 for hard AI
        this.winner = null; // 0 for O win, 1 for T win, 2 for draw, 3 for tie
        this.turn = 'T'; // T for Toot, O for Otto
        this.playerTok = playerTok;
        this.ai = this.mode !== null ? new OttoBot(this.get_mode(this.mode), this.playerTok == 'T' ? 'O' : 'T') : null;
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

export function drawBoardToot(size, mode, playerTok) {
    var rows, cols, gameBoard
    var game = new GameBoard(size, mode, playerTok);
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
            cell.appendChild(input);
            row.appendChild(cell);
        }
        table.appendChild(row);
    }
    gameBoard.appendChild(table);
}


function startTurn(cell_selected, game) {
    var maxRows = game.board.height();
    var maxCols = game.board.width();
    var selectedColumn = parseInt(cell_selected.substring(1), 10) % maxCols;
    var cellId = getEmptyCell(selectedColumn, maxRows, maxCols);

    if (cellId == -1 || game.board.allows_move(selectedColumn) === false){
        return;
    }

    performMove(cellId, game);

    // AI move
    if (!endGame(game) && game.mode != 0) {
        getAIMove(game);
    }
}

function endGame(game) {
    if (!game.board.is_terminal()) {
        return false;
    }

    let result = game.board.has_winner();
    if (result === 'w') {
        //there is a winner, we should get winner
        let winner = game.board.get_winner();
        if (winner === 'T') {
            game.winner = 1;
            alert("TOOT HAS WON!");
        } else {
            game.winner = 0;
            alert("OTTO HAS WON!");
        }
    }else if (result === 'd') {
        game.winner = 2;
        alert("DRAW!");
    }else if (result === 't') {
        game.winner = 3;
        alert("TIE!");
    }

    return true;
}

function getAIMove(game) {
    console.log("AI move");
    var maxRows = game.board.height();
    var maxCols = game.board.width();
    var ai_token = game.playerTok == 'T' ? 'O' : 'T';
    console.log("AI token: " + ai_token);
    var selectedColumnandtoken= game.ai.best_move(game.board, ai_token);
    console.log("AI move: " + selectedColumnandtoken);
    var selectedColumn = parseInt(selectedColumnandtoken.substring(0,1), 10);
    var token = selectedColumnandtoken.substring(1,2);
    var cellId = getEmptyCell(selectedColumn, maxRows, maxCols);

    console.log("AI move: " + cellId);
    console.log("AI token: " + token);

    if (cellId == -1) {
        return;
    }
    
    performMoveAI(cellId,token, game);
    
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
        
    let col = parseInt(cellId.substring(1), 10) % game.width;

    if (piece == 'T') {
        cell.classList.add('toot-token');
        cell.value = piece;
    } else {
        cell.classList.add('otto-token');
        cell.value = piece;
    }

    var turn = game.turn == 'T' ? 'T' : 'O';
    console.log("Turn " + turn)
    game.board.perform_move_plz(col, piece, turn);
    console.log("Player move: " + cellId + " " + piece);
    game.nextTurn();
}


function performMoveAI(cellId, ai_token, game) {
    //funciton to hanlde AI move
    let cell = document.getElementById(cellId);
    cell.classList.remove('empty-cell');
    var piece = ai_token;
        
    let col = parseInt(cellId.substring(1), 10) % game.width;

    if (piece == 'T') {
        cell.classList.add('toot-token');
        cell.value = piece;
    } else {
        cell.classList.add('otto-token');
        cell.value = piece;
    }

    var turn = game.turn == 'O' ? 'O' : 'T';
    console.log("AI Turn " + turn)
    game.board.perform_move_plz(col, piece, turn);
    game.nextTurn();
}
