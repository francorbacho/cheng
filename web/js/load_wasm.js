import init, { get_pieces } from '/pkg/cheng.js';

async function run() {
    await init();

    window.cheng = {};
    window.cheng.get_pieces = get_pieces;

    mainBoard.constructHTML();
}

run();