class Chessboard {
    constructor(id) {
        this.boardFrameId = id;
        this.squares = {};
        this.draggingPiece = null;
    }

    constructBoard() {
        for (let i = 1; i <= 8; i++) {
            for (let j = 1; j <= 8; j++) {
                const position = `${String.fromCharCode(j + 96)}${i}`;

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
            }
        }
    }

    constructPieces() {
        const pieces = cheng.get_pieces();

        for (let piece of pieces) {
            const pieceElement = document.createElement('piece');
            pieceElement.classList.add(piece.piece);
            pieceElement.classList.add(piece.side);

            pieceElement.addEventListener("mousedown", event => this.handlePieceDragStart(event));
            pieceElement.addEventListener("touchstart", event => this.handlePieceDragStart(event));
            pieceElement.addEventListener("mouseup", event => this.handlePieceDragEnd(event));
            pieceElement.addEventListener("touchend", event => this.handlePieceDragEnd(event));

            const square = document.querySelector(`square[position=${piece.position}]`);
            square.appendChild(pieceElement);
        }

        this.boardFrame.addEventListener("mousemove", event => this.handlePieceDrag(event));
        this.boardFrame.addEventListener("touchmove", event => this.handlePieceDrag(event));
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
        if (event.type === "touchstart") {
            event.preventDefault();
        }

        this.draggingPiece = event.target;
        this.draggingPiece.classList.add("dragging");
    }


    handlePieceDrag(event) {
        if (!this.draggingPiece) return;
        const clientX = event.clientX ?? event.touches[0].clientX;
        const clientY = event.clientY ?? event.touches[0].clientY;

        const elementStyle = getComputedStyle(this.draggingPiece);
        const elementWidth = Number.parseFloat(elementStyle.width);
        const elementHeight = Number.parseFloat(elementStyle.height);

        const newX = clientX - elementWidth / 2;
        const newY = clientY - elementHeight / 2;

        this.draggingPiece.style.left = `${newX}px`;
        this.draggingPiece.style.top = `${newY}px`;
    }

    handlePieceDragEnd(event) {
        const movedPiece = this.draggingPiece;
        this.draggingPiece.classList.remove('dragging');
        this.draggingPiece = null;

        movedPiece.style.top = "";
        movedPiece.style.left = "";

        const clientX = event.clientX || event.changedTouches[0].clientX;
        const clientY = event.clientY || event.changedTouches[0].clientY;

        const board = document.querySelector("chessboard");
        const boardRect = board.getBoundingClientRect();

        if (boardRect.top > clientY || boardRect.bottom < clientY) {
            return;
        }

        if (boardRect.left > clientX || boardRect.right < clientY) {
            return;
        }

        const x = clientX - boardRect.left;
        const y = boardRect.height - (clientY - boardRect.top);

        const { width, height } = document.querySelector('square').getBoundingClientRect();
        const column = Math.floor(x / width) + 1;
        const row = Math.floor(y / height) + 1;

        const position = `${String.fromCharCode(column + 96)}${row}`;
        const destSquare = document.querySelector(`square[position=${position}]`);

        if (!destSquare) { return; }

        destSquare.appendChild(movedPiece);

        const now = new Date();
        document.getElementById('state').textContent = `dragging piece ended @ ${now.getHours()}:${now.getMinutes()}:${now.getSeconds()}.${now.getMilliseconds()}`;
    }
}

const mainBoard = new Chessboard('chessboard');