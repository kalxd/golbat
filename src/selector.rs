use scraper::Selector;
use std::ffi::{c_char, CStr};

use crate::rule::drop_ptr;

#[no_mangle]
pub extern "C" fn build_selector(char_ptr: *const c_char) -> *const Selector {
	let s = unsafe { CStr::from_ptr(char_ptr) }.to_str().unwrap();
	let selector = Selector::parse(s).unwrap();
	crate::into_ptr!(selector)
}

#[no_mangle]
pub unsafe extern "C" fn free_selector(ptr: *mut Selector) {
	drop_ptr(ptr)
}
