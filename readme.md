# FFIString

This is a small crate for moving strings across `extern "C"`, giving FFI-safe version of String and &str

<br>

### This crate contains two new types: [`FFIString`](https://docs.rs/ffi-string/latest/ffi_string/struct.FFIString.html) and [`FFIStr`](https://docs.rs/ffi-string/latest/ffi_string/struct.FFIStr<'a>.html)

These work just like `String` and `&'a str`, but with `#[repr(C)]`

<br>

If you have any ideas on how this can be improved, please submit an issue on the repository
