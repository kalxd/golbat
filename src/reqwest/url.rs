use reqwest::Url;
use std::ffi::{CString, c_char};

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

#[unsafe(no_mangle)]
extern "C" fn url_has_host(host: *const c_char, url: *const Url) -> bool {
	let url = unsafe { &*url };
	let host = unsafe { unsafe_str(host) };

	url.host_str() == Some(host)
}

#[unsafe(no_mangle)]
extern "C" fn url_filename(url: *const Url) -> *const c_char {
	let url = unsafe { &*url };

	let filepath = url.to_file_path().ok();
	let filepath = filepath.as_ref().and_then(|p| p.to_str());

	match filepath {
		Some(path) => CString::new(path).unwrap().into_raw(),
		None => std::ptr::null(),
	}
}
