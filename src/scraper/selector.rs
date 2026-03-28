use scraper::Selector;
use std::ffi::c_char;

use crate::result::CResult;
use crate::rule::{drop_ptr, unsafe_str};
use crate::{not_ok, ok};

#[unsafe(no_mangle)]
pub extern "C" fn selector_create(char_ptr: *const c_char) -> *mut CResult<*mut Selector> {
	let s = unsafe { unsafe_str(char_ptr) };
	match Selector::parse(s) {
		Ok(selector) => ok!(selector),
		Err(e) => not_ok!(e.to_string()),
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn selector_free(ptr: *mut Selector) {
	unsafe { drop_ptr(ptr) }
}
