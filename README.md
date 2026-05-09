# Wgpu + Wesl + Winit + Egui + WebAssembly template

This is a template repository to get started with wgpu and egui, also working in webassembly.

## Versions

| Wgpu | Egui   | Winit  | Wesl  |
| ---- | ------ | ------ | ----- |
| `29` | `0.34` | `0.30` | `0.3` |

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
