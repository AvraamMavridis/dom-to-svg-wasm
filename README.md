
#### Rust

```bash
curl https://sh.rustup.rs -sSf | sh

rustup default nightly
```


Setup
```
rustup target add wasm32-unknown-unknown
cargo +nightly install wasm-bindgen-cli
```

```
npm install
```


Build `wasm` module:

```
cargo build --target wasm32-unknown-unknown
```

Running FE:

```
npm run debug-build

npm run serve
```
