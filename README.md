# cheng - Chess Engine
Stupid Bad Chess Engine, in Rust.

![Screenshot of Firefox running the web front-end](/repo/screenshot_2023-08-21.png)

## Running
The Rust codebase is separated into three different crates: `cheng` (the engine
itself); `cheng-cmd`, a binary capable of interacting and debugging the engine,
and an attempt to build a UCI-compliant program in the future; and `chess-wasm`,
which when combined with the sources in `web`, provide a simple web front-end.

To run the CLI utility, run:
```bash
$ cargo +nightly run --release -p cheng-cmd
```

To run the web front-end, you'll need
[wasm-pack](https://rustwasm.github.io/docs/wasm-pack/), which can be
installed with `cargo install wasm-pack`. Add it to `$PATH`, then run:
```bash
$ cd web/ && ./deploy.sh && python -m http.server
```
