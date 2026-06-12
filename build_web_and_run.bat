@echo off
rem cargo install wasm-bindgen-cli

pushd "%~dp0"

cargo +nightly build --lib --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort
wasm-bindgen --target web --out-dir web/pkg/wasm target/wasm32-unknown-unknown/release/rubiks_cube_simulation.wasm
python web/server.py web/pkg

popd
