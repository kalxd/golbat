use scraper::Selector;
use std::ffi::{c_char, CStr};

#[no_mangle]
pub extern "C" fn build_selector(char_ptr: *const c_char) -> *const Selector {
	let s = unsafe { CStr::from_ptr(char_ptr) }.to_str().unwrap();
	let selector = Selector::parse(s).unwrap();
	crate::into_ptr!(selector)
}
