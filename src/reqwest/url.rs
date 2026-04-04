use reqwest::Url;
use std::ffi::{CString, c_char};

use crate::{
	into_ptr, into_result,
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

// # panic
// 直接假定input就是合法url！
#[unsafe(no_mangle)]
extern "C" fn url_unsafe_parse(input: *const c_char) -> *const Url {
	let url = unsafe { unsafe_str(input) };
	let url = Url::parse(url).expect(&format!("{url}不是有效的URL！"));
	into_ptr!(url)
}

// 不会改变原有url，set_path会生成一条新的url。
#[unsafe(no_mangle)]
extern "C" fn url_set_path(url: *const Url, path: *const c_char) -> *const Url {
	let path = unsafe { unsafe_str(path) };
	let mut url = unsafe { &*url }.clone();

	url.set_path(path);
	into_ptr!(url)
}

#[unsafe(no_mangle)]
extern "C" fn url_contain_host(host: *const c_char, url: *const Url) -> u8 {
	let url = unsafe { &*url };
	let host = unsafe { unsafe_str(host) };

	if url.host_str() == Some(host) { 1 } else { 0 }
}

#[unsafe(no_mangle)]
extern "C" fn url_file_path(url: *const Url) -> *const c_char {
	let url = unsafe { &*url };

	let path = url.path_segments().and_then(|sp| sp.last());

	match path {
		Some(path) => CString::new(path).unwrap().into_raw(),
		None => std::ptr::null(),
	}
}
