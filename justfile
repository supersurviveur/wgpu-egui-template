web:
    wasm-pack build --target web
    python -m http.server

host:
    cargo r
