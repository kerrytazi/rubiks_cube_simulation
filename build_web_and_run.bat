@echo off
rem cargo install wasm-bindgen-cli

pushd "%~dp0"

cargo +nightly build --package web --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort
wasm-bindgen --target web --out-dir crates/web/pkg/wasm target/wasm32-unknown-unknown/release/web.wasm
python crates/web/server.py crates/web/pkg

popd
