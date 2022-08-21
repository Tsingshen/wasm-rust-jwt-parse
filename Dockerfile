FROM scratch


COPY target/wasm32-unknown-unknown/release/wasm_rust_jwt_parse.wasm ./plugin.wasm