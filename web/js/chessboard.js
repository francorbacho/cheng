class Chessboard {
    constructor(id) {
        this.boardFrameId = id;
        this.squares = {};
    }

    constructBoard() {
        for (let i = 1; i <= 8; i++) {
            for (let j = 1; j <= 8; j++) {
                let position = `${String.fromCharCode(j + 96)}${i}`;

                const squareElement = document.createElement('square');
                squareElement.setAttribute('position', position);
                squareElement.draggable = false;

                if ((i + j) % 2 == 1) {
                    squareElement.classList.add('white');
                } else {
                    squareElement.classList.add('black');
                }

                this.squares[position] = squareElement;
                this.boardFrame.appendChild(squareElement);

                squareElement.addEventListener('drop', this.handleSquareDrop);
                squareElement.addEventListener('dragover', this.handleSquareDragOver);
            }
        }
    }

    constructPieces() {
        const pieces = cheng.get_pieces();

        for (let piece of pieces) {
            const pieceElement = document.createElement('piece');
            pieceElement.classList.add(piece.piece);
            pieceElement.classList.add(piece.side);
            pieceElement.draggable = true;

            pieceElement.addEventListener('dragstart', this.handlePieceDragStart);
            pieceElement.addEventListener('dragend', this.handlePieceDragEnd);

            const square = document.querySelector(`square[position=${piece.position}]`);
            square.appendChild(pieceElement);
        }
    }

    constructHTML() {
        this.boardFrame = document.getElementById(this.boardFrameId);

        if (!this.boardFrame) {
            throw new Error(`Element with id=${id} not found.`);
        }

        this.constructBoard();
        this.constructPieces();
    }

    handlePieceDragStart(event) {
        const position = event.target.parentElement.getAttribute('position');
        event.dataTransfer.setData('text/plain', position);
        event.target.classList.add('dragging');
    }

    handlePieceDragEnd(event) {
        event.target.classList.remove('dragging');
    }

    handleSquareDragOver(event) {
        event.preventDefault();
    }

    handleSquareDrop(event) {
        event.preventDefault();

        const piecePosition = event.dataTransfer.getData('text/plain');
        const targetSquare = event.target.closest('square');
        const targetPosition = targetSquare.getAttribute('position');

        const originSquareObj = document.querySelector(`square[position=${piecePosition}]`);
        const targetSquareObj = document.querySelector(`square[position=${targetPosition}]`);

        const pieceElement = originSquareObj.children[0];
        const targetElement = targetSquareObj.children[0];

        if (pieceElement && !targetElement) {
            targetSquare.appendChild(pieceElement);
        }
    }
}

const mainBoard = new Chessboard('chessboard');