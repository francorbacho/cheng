// from https://dev.to/dandyvica/wasm-in-rust-without-nodejs-2e0c
WebAssembly.instantiateStreaming(fetch("wasm/cheng.wasm"))
    .then(onWasmLoad, onWasmLoadRejected);

function onWasmLoad(wasmModule) {
    console.log('WASM Loaded successfuly');

    wasm_init = wasmModule.instance.exports.initialize;
    wasm_get_pawn_count = wasmModule.instance.exports.get_pawn_count;
}

function onWasmLoadRejected() {
    console.error('Failed to load WASM');
}