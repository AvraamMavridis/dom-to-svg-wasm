# rustpdf (using wasm)
Attempt at learning, web assembly with rust. 


## Pre-requisite

#### Rust

```bash
curl https://sh.rustup.rs -sSf | sh

rustup default nightly
```

### Node/npm



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


Resources:

- Tensor Programming's Rust Webassembly series: https://www.youtube.com/watch?v=yEiGVCF99tA
- https://www.fullstackreact.com/articles/rust-react-and-web-assembly/
- https://hacks.mozilla.org/2018/04/javascript-to-rust-and-back-again-a-wasm-bindgen-tale/
