import init, * as wasm from '../pkg/chess_wasm.js';

self.onmessage = async (event) => {
    await init();

    const { inputData } = event.data;
    wasm.loadBoardFromFen(inputData);

    const result = await wasm.flimsybirdRun().catch(() => { /* TODO: Handle this? */ });
    self.postMessage(result);
};
