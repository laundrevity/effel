Hello. This is an example showing how to use Bevy to render shaders in WebAssembly.

<h2> Prerequisites </h2>
Make sure you have Rust installed (google rustup and execute that shell command if not). Then you can install the other dependencies in your Rust environment with:

```
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo install basic-http-server
```

Next, to compile the source code into WebAssembly, you can use 
```
cargo build --release --target wasm32-unknown-unknown
```
and then
```
wasm-bindgen --out-name wasm_example \
  --out-dir wasm/target --target \
  --target web target/wasm32-unknown-unknown/release/effel.wasm
```

Make sure to symbolically link your assets into the `wasm` directory, like this
```
ln -s ../assets wasm/assets
```

Now, you can serve the `wasm` directory (which contains the JS bindings and a simple `index.html` importing it) via 
```
basic-http-server wasm
```