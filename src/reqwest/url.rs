use reqwest::Url;
use std::ffi::c_char;

use crate::{
	into_result,
	result::CResult,
	rule::{drop_ptr, unsafe_str},
};

#[unsafe(no_mangle)]
extern "C" fn url_dbg(url: *const Url) {
	let url = unsafe { &*url };
	dbg!(&url);
}

#[unsafe(no_mangle)]
extern "C" fn url_free(url: *mut Url) {
	unsafe { drop_ptr(url) }
}

#[unsafe(no_mangle)]
extern "C" fn url_parse(input: *const c_char) -> *const CResult<*const Url> {
	let url = unsafe { unsafe_str(input) };
	into_result!(Url::parse(url))
}
