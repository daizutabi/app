# app

```bash
cargo install cargo-nextest
cargo nextest run
cargo nextest run -p hello
cargo nextest run -p src-ui
```

```bash
cargo install wasm-pack
wasm-pack test --chrome apps/src-ui
wasm-pack test --chrome --headless apps/src-ui --test counter
```

```bash
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview
cargo llvm-cov --lcov --output-path lcov.info
```
