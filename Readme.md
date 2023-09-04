Project to learn WASM + Rust

goal: make a simple ball rolling simulator -> fluid simulator

References

[Rust lang install](https://www.rust-lang.org/learn/get-started)

[Rust WASM Game of life](https://rustwasm.github.io/docs/book/game-of-life)
- some issues with template, but following [mdn](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm) with bootstraping fixed some of the packaging issues


Basic setup and running:
From $ compute-engine
$ wasm-pack build --target bundler
$ cd pkg && npm link
$ cd ../site npm link compute-engine
$ npm run serve




webpack 4 error

Run


https://github.com/webpack/webpack/issues/14532
export NODE_OPTIONS=--openssl-legacy-provider

