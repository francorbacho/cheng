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
        const pieces = wasm.getPieces();

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
        this.boardFrame.textContent = "";

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

        const boardState = wasm.getState();
        if (boardState.result) {
            return;
        }

        if (!event.target.classList.contains(wasm.getSideToMove())) {
            return;
        }

        this.draggingPiece = event.target;
        this.draggingPiece.classList.add("dragging");
    }


    handlePieceDrag(event) {
        if (!this.draggingPiece) return;
        const pageX = event.pageX ?? event.touches[0].pageX;
        const pageY = event.pageY ?? event.touches[0].pageY;

        const elementStyle = getComputedStyle(this.draggingPiece);
        const elementWidth = Number.parseFloat(elementStyle.width);
        const elementHeight = Number.parseFloat(elementStyle.height);

        const newX = pageX - elementWidth / 2;
        const newY = pageY - elementHeight / 2;

        this.draggingPiece.style.left = `${newX}px`;
        this.draggingPiece.style.top = `${newY}px`;
    }

    handlePieceDragEnd(event) {
        if (!this.draggingPiece) return;

        const movedPiece = this.draggingPiece;
        this.draggingPiece.classList.remove('dragging');
        this.draggingPiece = null;

        movedPiece.style.top = "";
        movedPiece.style.left = "";

        // Fields clientX and clientY are used here because it is checked
        // against the result of getBoundingClientRect, which is also relative
        // to the window.
        const clientX = event.clientX || event.changedTouches[0].clientX;
        const clientY = event.clientY || event.changedTouches[0].clientY;

        const board = document.querySelector("chessboard");
        const boardRect = board.getBoundingClientRect();

        if (boardRect.top > clientY || boardRect.bottom < clientY) {
            return;
        }

        if (boardRect.left > clientX || boardRect.right < clientX) {
            return;
        }

        const x = clientX - boardRect.left;
        const y = boardRect.height - (clientY - boardRect.top);

        const { width, height } = document.querySelector('square').getBoundingClientRect();
        const column = Math.floor(x / width) + 1;
        const row = Math.floor(y / height) + 1;

        const destSquare = `${String.fromCharCode(column + 96)}${row}`;
        const destSquareElement = document.querySelector(`square[position=${destSquare}]`);

        const sourceSquareElement = movedPiece.parentElement;
        const sourceSquare = sourceSquareElement.getAttribute('position');

        if (!destSquareElement || sourceSquareElement == destSquareElement) { return; }

        const movement = `${sourceSquare}${destSquare}`;
        try {
            this.feedMove(movement);
        } catch (exception) {
            console.error(exception);
            return;
        }

        const now = new Date();
        document.getElementById('state').textContent = `dragging piece ended @ ${now.getHours()}:${now.getMinutes()}:${now.getSeconds()}.${now.getMilliseconds()}`;
    }

    feedMove(movement) {
        const moveFeedback = wasm.feedMove(movement);

        const originSquareElement = document.querySelector(`square[position=${moveFeedback.origin}]`);
        const destSquareElement = document.querySelector(`square[position=${moveFeedback.destination}]`);
        const movedPiece = originSquareElement.children[0];

        if (moveFeedback.moveIsCapture) {
            destSquareElement.children[0].remove();
        }

        if (moveFeedback.castleSide) {
            const rookOriginSquareElement = document.querySelector(`square[position=${moveFeedback.rookSquareBeforeCastle}]`);
            const rookElement = rookOriginSquareElement.children[0];

            const rookDestSquareElement = document.querySelector(`square[position=${moveFeedback.rookSquareAfterCastle}]`);
            rookDestSquareElement.appendChild(rookElement);
        }

        destSquareElement.appendChild(movedPiece);

        this.updateCheckIndicator();
        this.updatePreviousMoveIndicator(moveFeedback.origin, moveFeedback.destination);
    }

    updateCheckIndicator() {
        const markElement = document.querySelector(`mark.check`);
        if (markElement) markElement.remove();

        const boardState = wasm.getState();
        if (boardState.result == 'checkmate') {
            const checkmateMark = document.createElement('mark');
            const kingElement = document.querySelector(`.${wasm.getSideToMove()}.king`);

            checkmateMark.classList.add('checkmate');
            kingElement.appendChild(checkmateMark);
        } else if (boardState.kingInCheck) {
            const checkMark = document.createElement('mark');
            const kingElement = document.querySelector(`.${wasm.getSideToMove()}.king`);
            checkMark.classList.add('check');
            kingElement.appendChild(checkMark);
        }
    }

    updatePreviousMoveIndicator(newMoveOrigin, newMoveDestination) {
        const lastMoveSquareElement = document.querySelectorAll('square.last-move');

        for (const squareElement of lastMoveSquareElement) {
            squareElement.classList.remove('last-move');
        }

        const originSquareElement = document.querySelector(`square[position=${newMoveOrigin}]`);
        const destSquareElement = document.querySelector(`square[position=${newMoveDestination}]`);

        originSquareElement.classList.add('last-move');
        destSquareElement.classList.add('last-move');
    }
}

const mainBoard = new Chessboard('chessboard');