import init, * as wasm from '../pkg/chess_wasm.js';

async function run() {
    await init();

    window.wasm = wasm;

    mainBoard.constructHTML();
}

run();
