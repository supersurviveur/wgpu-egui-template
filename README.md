# Wgpu + Winit + Egui + WebAssembly template

This is a template repository to get started with wgpu and egui, also working in webassembly.

## Versions

| Wgpu | Egui   | Winit  |
| ---- | ------ | ------ |
| `27` | `0.33` | `0.30` |

## Get Started

```sh
git clone https://github.com/supersurviveur/wgpu-egui-template.git
cd wgpu-egui-template

# Locally
cargo run --release

# Or in WASM
wasm-pack build --target web
python -m http.server
# Then go to http://localhost:8000/
```
