## Rust Runtime Example

### Set up project

```shell
# create an lib(in this example, we use library setup)
cargo init --lib
# create an executable
cargo build --target=wasm32-unknown-unknown --release
```
![wasm](./images/wasm.png)


### Setup WebAssembly Binary Toolkit

- setup project https://github.com/WebAssembly/wabt
  提供了一些有用的工具：
- wasm2wat

### Setup

- setup project https://github.com/WebAssembly/binaryen

用于快速，高效地编译WebAssembly. 提供的wasm-dis，类似于wasm-opt. 但是wasm-dis生成的是WAST(WebAssembly S-Expression Text Format),
而wasm-opt生成的是标准的WAT(WebAssembly Text Format).

### Info

WebAssembly 通常被作为library，但是在[WASI](https://wasi.dev/) 的上下文中会作为executable。
