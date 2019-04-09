# simple-wasm

Sample Rust `wasm-pack` project that depends transitively on C code.

- [`double-rs`](double-rs): high-level bindings around `double-sys`
- [`double-sys`](double-sys): low-level bindings around C library `double`
- [`double-sys/double`](double-sys/double): C library that can double an integer's value
