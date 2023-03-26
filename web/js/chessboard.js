function pieceName(piece) {
    const pieces = {
        'p': 'pawn',
        'n': 'knight',
        'b': 'bishop',
        'r': 'rook',
        'q': 'queen',
        'k': 'king',
    };

    return pieces[piece.toLowerCase()];
}

function pieceColor(piece) {
    if ('pnbrqk'.includes(piece)) {
        return 'black';
    } else if ('PNBRQK'.includes(piece)) {
        return 'white';
    } else {
        return 'unknown';
    }
}

class Chessboard {
    constructor() {
        this.board = [
            ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
            ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
            ['', '', '', '', '', '', '', ''],
            ['', '', '', '', '', '', '', ''],
            ['', '', '', '', '', '', '', ''],
            ['', '', '', '', '', '', '', ''],
            ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
            ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
        ];
    }

    constructHTMLBoard() {
        const chessboard = document.getElementById('chessboard');
        for (let i = 0; i < 8; i++) {
            for (let j = 0; j < 8; j++) {
                const square = document.createElement('square');
                square.draggable = "false";

                if ((i + j) % 2 === 0) {
                    square.className = 'white';
                } else {
                    square.className = 'black';
                }

                const piece = this.board[i][j];
                if (piece) {
                    const pieceElem = document.createElement('piece');
                    pieceElem.className = `${pieceColor(piece)} ${pieceName(piece)}`;

                    pieceElem.addEventListener('mousedown', onPieceMouseDown);

                    square.appendChild(pieceElem);
                }
                chessboard.appendChild(square);
            }
        }

        chessboard.addEventListener('mousemove', onMouseMove);
        chessboard.addEventListener('mouseup', onPieceMouseRelease);
    }
}

window.onload = function () {
    const chessboard = new Chessboard();
    chessboard.constructHTMLBoard();
};

let followingTarget = null;
let mouseDragStartingPosition = null;

function onPieceMouseDown(event) {
    followingTarget = event.target;

    mouseDragStartingPosition = {};
    mouseDragStartingPosition.clientX = event.clientX;
    mouseDragStartingPosition.clientY = event.clientY;
}

function onPieceMouseRelease(event) {
    followingTarget.style = 'transform: translate(0px);';
    followingTarget = null;
    mouseDragStartingPosition = null;
}

function onMouseMove(event) {
    if (!followingTarget) {
        return;
    }

    const x = event.clientX - mouseDragStartingPosition.clientX;
    const y = event.clientY - mouseDragStartingPosition.clientY;
    followingTarget.style = `transform: translate(${x}px, ${y}px);`;
}
