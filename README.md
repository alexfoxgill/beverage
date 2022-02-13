To run: `cargo run`

To run (web):

1. (in .) `wasm-pack build --target web --release`
2. (in ./pkg) `python3 -m http.server`
3. (in browser) `http://localhost:8000`