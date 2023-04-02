import init, * as wasm from '/pkg/cheng.js';

async function run() {
    await init();

    window.wasm = wasm;

    mainBoard.constructHTML();
}

run();