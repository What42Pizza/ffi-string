## 1.0.4
24/12/28

- Fix wording

## 1.0.3
24/12/28

- Improved documentation
- Removed `to_ffi_string()` from `StringToFFI` because it is already available with String's `Deref<Target = str>`

<br>

## 1.0.2
24/12/20

- Added `to_ffi_str()` and `to_ffi_string()` to &str
- Added `to_ffi_string()` and `into_ffi_string()` to String

<br>

## 1.0.1
24/12/20

- Added `Debug` to both types
- Added `Clone` to both types
- Added `Copy` to FFIStr

<br>

## 1.0.0
24/12/20

- Initial creation and release
