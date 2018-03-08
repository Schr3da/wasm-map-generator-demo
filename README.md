# hackertron-eu-wasm-game-project
Source code of Rust/WASM game development dev blog (http://www.hackertron.eu)

# Current WebAssembly build 
http://www.hackertron.eu/build/

# WebAssembly Build Instruction/Requirements

* Proper workin emscripten environment
* Use set_project_path.sh script to update required template variables in .cargo/config
* Use cargo build --target wasm32-unknown-emscripten --release
* Copy the following files into the template folder:
 * target/wasm32-unknown-emscripten/debug/deps/game.js
 * target/wasm32-unknown-emscripten/debug/deps/game.wasm
 * target/wasm32-unknown-emscripten/debug/deps/game.data
 * target/wasm32-unknown-emscripten/debug/deps/game.d
* Use static file of choice to host template folder e.g http-server
 * npm install http-server -g
 * http-server ./template -p 9000
 * Open browser http://127.0.0.1:9000
