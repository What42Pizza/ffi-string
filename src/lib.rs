//! # FFI String
//! 
//! This is a small crate for moving strings across `extern "C"`, giving FFI-safe version of String and &str
//! 



#![feature(str_from_raw_parts)]

#![warn(missing_docs)]



use std::ops::Deref;



/// FFI version of &str
#[repr(C)]
pub struct FFIStr<'a> {
	ptr: &'a u8,
	len: u32,
}

impl<'a> FFIStr<'a> {
	/// Creates a new FFIStr from a string slice, copying only pointers
	pub fn new(from: &'a str) -> Self {
		unsafe {
			Self {
				ptr: &*from.as_ptr(),
				len: from.len() as u32,
			}
		}
	}
	/// Creates a string slice, copying only pointers
	/// 
	/// The function `to_string()` (implementation of fmt::Display) creates a new String, copying the underlying data
	pub fn as_str(&self) -> &'a str {
		unsafe {
			core::str::from_raw_parts(self.ptr, self.len as usize)
		}
	}
}

impl Deref for FFIStr<'_> {
	type Target = str;
	fn deref(&self) -> &Self::Target {
		self.as_str()
	}
}

impl std::fmt::Display for FFIStr<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

impl<'a> From<&'a str> for FFIStr<'a> {
	fn from(value: &'a str) -> Self {
		Self::new(value)
	}
}

impl<'a> AsRef<str> for FFIStr<'a> {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl Into<String> for FFIStr<'_> {
	fn into(self) -> String {
		self.to_string()
	}
}



/// FFI version of String
#[repr(C)]
pub struct FFIString {
	ptr: *mut u8,
	len: u32,
	cap: u32,
}

impl FFIString {
	/// Creates a new FFIString from a String, copying only pointers (if you pass String) or all underlying data (for anything else)
	pub fn new(from: impl Into<String>) -> Self {
		let mut from = from.into();
		let output = Self {
			ptr: from.as_mut_ptr(),
			len: from.len() as u32,
			cap: from.capacity() as u32,
		};
		std::mem::forget(from);
		output
	}
	/// Creates a new String, copying only pointers
	/// 
	/// The function `to_string()` (implementation of fmt::Display) creates a new String, copying the underlying data
	pub fn into_string(self) -> String {
		unsafe {
			let output = String::from_raw_parts(self.ptr, self.len as usize, self.cap as usize);
			std::mem::forget(self);
			output
		}
	}
	/// Creates a string slice, copying only pointers
	pub fn as_str(&self) -> &str {
		unsafe {
			core::str::from_raw_parts(self.ptr, self.len as usize)
		}
	}
}

impl Drop for FFIString {
	fn drop(&mut self) {
		unsafe {
			String::from_raw_parts(self.ptr, self.len as usize, self.cap as usize);
		}
	}
}

impl Deref for FFIString {
	type Target = str;
	fn deref(&self) -> &Self::Target {
		self.as_str()
	}
}

impl std::fmt::Display for FFIString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

impl From<String> for FFIString {
	fn from(value: String) -> Self {
		Self::new(value)
	}
}

impl AsRef<str> for FFIString {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl Into<String> for FFIString {
	fn into(self) -> String {
		self.to_string()
	}
}
