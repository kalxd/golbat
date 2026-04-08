use scraper::Selector;
use std::ffi::c_char;

use crate::result::CResult;
use crate::rule::{drop_ptr, show, unsafe_str};
use crate::{not_ok, ok};

#[unsafe(no_mangle)]
extern "C" fn selector_create(char_ptr: *const c_char) -> *const CResult<*const Selector> {
	let s = unsafe { unsafe_str(char_ptr) };
	match Selector::parse(s) {
		Ok(selector) => ok!(selector),
		Err(e) => not_ok!(e.to_string()),
	}
}

#[unsafe(no_mangle)]
extern "C" fn selector_free(ptr: *mut Selector) {
	unsafe { drop_ptr(ptr) }
}

#[unsafe(no_mangle)]
extern "C" fn selector_show(ptr: *const Selector) -> *const c_char {
	let selector = unsafe { &*ptr };
	show(selector)
}
