class Chessboard {
    constructor(id) {
        this.boardFrameId = id;
        this.squares = {};
        this.draggingPiece = null;
        this.playerConfiguration = {
            white: new PlayerConfiguration("white", "human", document.getElementById("player-select-white"), document.getElementById("white-endpoint")),
            black: new PlayerConfiguration("black", "wasm", document.getElementById("player-select-black"), document.getElementById("black-endpoint")),
        };

        this.worker = new Worker("js/worker.js", { type: "module" });

        this.worker.onmessage = (ev) => { this.feedMove(ev.data); };
        this.worker.onerror = (ev) => {
            console.error("WORKER FAILED", ev);
        };
    }

    constructHTML() {
        this.boardFrame = document.getElementById(this.boardFrameId);
        this.boardFrame.textContent = "";

        if (!this.boardFrame) {
            throw new Error(`Element with id=${this.boardFrameId} not found.`);
        }

        this.constructBoard();
        this.constructPieces();
    }

    constructBoard() {
        for (let i = 1; i <= 8; i++) {
            for (let j = 1; j <= 8; j++) {
                const position = `${String.fromCharCode(j + 96)}${i}`;

                const squareElement = document.createElement("square");
                squareElement.setAttribute("position", position);
                squareElement.draggable = false;

                if ((i + j) % 2 == 1) {
                    squareElement.classList.add("white");
                } else {
                    squareElement.classList.add("black");
                }

                this.squares[position] = squareElement;
                this.boardFrame.appendChild(squareElement);
            }
        }
    }

    syncToWasm() {
        for (let i = 1; i <= 8; i++) {
            for (let j = 1; j <= 8; j++) {
                const position = `${String.fromCharCode(j + 96)}${i}`;

                while (this.squares[position].children.length > 0) {
                    this.squares[position].children[0].remove();
                }
            }
        }

        this.constructPieces();
        this.updateCheckIndicator();
        this.unsetPreviousMoveIndicator();
        this.updateFenInputBox();
    }

    constructPieces() {
        const pieces = wasm.getPieces();

        for (let piece of pieces) {
            const pieceElement = document.createElement("piece");
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

    handlePieceDragStart(event) {
        if (event.type === "touchstart") {
            event.preventDefault();
        }

        const boardState = wasm.getState();
        if (boardState.result) {
            return;
        }

        const pieceSide = event.target.classList.contains("white") ? "white" : "black";

        if (pieceSide != wasm.getSideToMove()) {
            return;
        }

        if (!this.playerConfiguration[pieceSide].allowDrag()) {
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
        this.draggingPiece.classList.remove("dragging");
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

        const { width, height } = document.querySelector("square").getBoundingClientRect();
        const column = Math.floor(x / width) + 1;
        const row = Math.floor(y / height) + 1;

        const destSquare = `${String.fromCharCode(column + 96)}${row}`;
        const destSquareElement = document.querySelector(`square[position=${destSquare}]`);

        const sourceSquareElement = movedPiece.parentElement;
        const sourceSquare = sourceSquareElement.getAttribute("position");

        if (!destSquareElement || sourceSquareElement == destSquareElement) { return; }

        const isPawn = movedPiece.classList.contains("pawn");
        const promotion = (isPawn && (row == 8 || row == 1)) ? "q" : "";
        const movement = `${sourceSquare}${destSquare}${promotion}`;

        try {
            this.feedMove(movement);
        } catch (exception) {
            console.error(exception);
            return;
        }
    }

    feedMoveWithoutScheduling(movement) {
        const moveFeedback = wasm.feedMove(movement);

        const originSquareElement = document.querySelector(`square[position=${moveFeedback.origin}]`);
        const destSquareElement = document.querySelector(`square[position=${moveFeedback.destination}]`);
        const movedPiece = originSquareElement.children[0];

        if (moveFeedback.moveIsCapture) {
            if (moveFeedback.passedEnPassantPawnSquare) {
                const passedEnPassantPawnSquareElement = document.querySelector(`square[position=${moveFeedback.passedEnPassantPawnSquare}]`);
                passedEnPassantPawnSquareElement.children[0].remove();
            } else {
                destSquareElement.children[0].remove();
            }
        }

        if (moveFeedback.castleSide) {
            const rookOriginSquareElement = document.querySelector(`square[position=${moveFeedback.rookSquareBeforeCastle}]`);
            const rookElement = rookOriginSquareElement.children[0];

            const rookDestSquareElement = document.querySelector(`square[position=${moveFeedback.rookSquareAfterCastle}]`);
            rookDestSquareElement.appendChild(rookElement);
        }

        if (moveFeedback.promotion) {
            movedPiece.classList.replace("pawn", moveFeedback.promotion);
        }

        destSquareElement.appendChild(movedPiece);

        this.updateCheckIndicator();
        this.updatePreviousMoveIndicator(moveFeedback.origin, moveFeedback.destination);
        this.updateFenInputBox();
    }

    feedMove(movement) {
        this.feedMoveWithoutScheduling(movement);
        const board = this;

        setTimeout(() => this.scheduleMove(), 500);
    }

    updateFenInputBox() {
        const fenInput = document.getElementById("fen");
        fenInput.value = wasm.boardToFen();
    }

    updateCheckIndicator() {
        const markElement = document.querySelector("mark.check");
        if (markElement) markElement.remove();

        const boardState = wasm.getState();
        if (boardState.result == "checkmate") {
            const checkmateMark = document.createElement("mark");
            const kingElement = document.querySelector(`.${wasm.getSideToMove()}.king`);

            checkmateMark.classList.add("checkmate");
            kingElement.appendChild(checkmateMark);
        } else if (boardState.kingInCheck) {
            const checkMark = document.createElement("mark");
            const kingElement = document.querySelector(`.${wasm.getSideToMove()}.king`);
            checkMark.classList.add("check");
            kingElement.appendChild(checkMark);
        }
    }


    unsetPreviousMoveIndicator() {
        const lastMoveSquareElement = document.querySelectorAll("square.last-move");

        for (const squareElement of lastMoveSquareElement) {
            squareElement.classList.remove("last-move");
        }
    }

    updatePreviousMoveIndicator(newMoveOrigin, newMoveDestination) {
        this.unsetPreviousMoveIndicator();

        const originSquareElement = document.querySelector(`square[position=${newMoveOrigin}]`);
        const destSquareElement = document.querySelector(`square[position=${newMoveDestination}]`);

        originSquareElement.classList.add("last-move");
        destSquareElement.classList.add("last-move");
    }

    scheduleMove() {
        if (this.playerConfiguration[wasm.getSideToMove()].kind === "wasm") {
            this.worker.postMessage({ inputData: wasm.boardToFen() });
            return;
        }

        if (this.playerConfiguration[wasm.getSideToMove()].kind === "endpoint") {
            const endpoint = this.playerConfiguration[wasm.getSideToMove()].endpoint();
            if (endpoint === null) {
                console.error("Cannot request endpoint cause endpoint is null");
                return;
            }

            const fen = wasm.boardToFen().replaceAll("/", "_").replaceAll(" ", "+");
            const url = `${endpoint}?fen=${fen}`;

            fetch(url)
                .then(resp => resp.json())
                .then(j => {
                    mainBoard.feedMove(j.movement);
                });
        }
    }
}

class PlayerConfiguration {
    constructor(color, kind, selectElement, endpointTextElement) {
        this.color = color;
        this.kind = kind;
        this.selectElement = selectElement;
        this.endpointTextElement = endpointTextElement;

        this.selectElement.value = kind;

        const playerConf = this;
        this.selectElement.addEventListener("change", function () {
            playerConf.kind = playerConf.selectElement.value;
            playerConf.syncEnabledState();
            mainBoard.scheduleMove();
        });

        // FIXME: We should have a reference to the board, instead of using the
        //        global one...
        this.endpointTextElement.addEventListener("change", function() {
            mainBoard.scheduleMove();
        });

        this.syncEnabledState();
    }

    allowDrag() {
        return this.kind === "human";
    }

    endpoint() {
        return this.endpointTextElement.value;
    }

    syncEnabledState() {
        if (this.kind === "endpoint") {
            this.endpointTextElement.disabled = false;
        } else {
            this.endpointTextElement.disabled = true;
        }
    }
}

window.onload = function () {
    window.mainBoard = new Chessboard("chessboard");

    const boardFrame = document.getElementById(mainBoard.boardFrameId);
    boardFrame.textContent = "Waiting for WebAssembly to load...";

    setTimeout(() => {
        if (typeof wasm == "undefined")
            boardFrame.textContent = "Failed to load WebAssembly.";
    }, 2_000);

    const fenInput = document.getElementById("fen");
    fenInput.addEventListener("change", function () {
        wasm.loadBoardFromFen(fenInput.value);
        mainBoard.syncToWasm();
    });

    const uciInput = document.getElementById("uci");
    uciInput.addEventListener("change", function () {
        const uciCommand = uciInput.value.trim().split(" ");

        if (uciCommand[0] !== "position" || uciCommand[1] !== "fen")
            throw new Error(`Expected \`position fen\` command, got \`${uciCommand[0]} ${uciCommand[1]}\``);

        const fen = uciCommand.slice(2, 8);
        wasm.loadBoardFromFen(fen.join(" "));
        mainBoard.syncToWasm();

        if (uciCommand[8] !== "moves")
            throw new Error(`Expected \`moves\` subcommand, got \`${uciCommand[8]}\``);

        const moves = uciCommand.slice(9);

        new Promise(async () => {
            for (let movement of moves) {
                await new Promise(r => setTimeout(r, 500));
                mainBoard.feedMoveWithoutScheduling(movement);
            }
        });
    });
};
