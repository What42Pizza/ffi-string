# FFIString

This is a small crate for moving strings across `extern "C"`, giving FFI-safe version of `String` and `&str`

### NOTE: You should also check out [Abi Stable](https://crates.io/crates/abi_stable), which I didn't know about when I made this crate

<br>

### This crate provides two types: [`FFIString`](https://docs.rs/ffi-string/latest/ffi_string/struct.FFIString.html) and [`FFIStr`](https://docs.rs/ffi-string/latest/ffi_string/struct.FFIStr<'a>.html)

These work just like `String` and `&'a str`, but with `#[repr(C)]`

<br>

### Why?

I'm trying to make a game engine that loads game code at runtime, and I want both sides to be written in Rust. Without `extern "C"`, both sides would need to be compiled with the exact same version of rustc, so I need an FFI-safe way to pass around `String` and `&str`

<br>

If you have any ideas on how this can be improved, please submit an issue on the repository
