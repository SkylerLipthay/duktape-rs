# duktape-rs

I found that all of the other Duktape bindings were either outdated or
incomplete for my needs.

## Rebuild Duktape bindings

```sh
cargo build --features=build-ffi-gen
C_INCLUDE_PATH=/usr/lib/clang/6.0/include target/debug/ffi-gen
```
