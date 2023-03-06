## Rust Runtime Example

### Set up project

```shell
# create an lib(in this example, we use library setup)
cargo init --lib
# create an executable
cargo build --target=wasm32-unknown-unknown --release
```

## Size
```shell
cargo size --target thumbv7m-none-eabi --bin rt
```
![zero-size](./images/zero-size.png)

```shell
rustup component add llvm-tools-preview
# shows an empty binary
cargo size --target thumbv7m-none-eabi --bin rust-embedded
# before linking, the crate contains the panicking symbol
cargo rustc --target thumbv7m-none-eabi -- --emit=obj
```

```shell
# should use cargo nm, but does not work as expected
/Users/mrzhao/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/bin/llvm-nm target/thumbv7m-none-eabi/debug/deps/rt-261c79ebd53b251c.o
```
![nm](images/nm.png)