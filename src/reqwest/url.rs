use reqwest::Url;
use std::ffi::c_char;

use crate::{
	into_ptr, into_result,
	result::CResult,
	rule::{drop_ptr, into_c_string, unsafe_str},
};

#[unsafe(no_mangle)]
extern "C" fn url_show(url: *const Url) -> *const c_char {
	let url = unsafe { &*url };
	let s = format!("{:?}", url);
	into_c_string(&s)
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

// 不会改变原有url，set_path会生成一条新的url。
#[unsafe(no_mangle)]
extern "C" fn url_set_path(path: *const c_char, url: *const Url) -> *const Url {
	let path = unsafe { unsafe_str(path) };
	let mut url = unsafe { &*url }.clone();

	url.set_path(path);
	into_ptr!(url)
}

#[unsafe(no_mangle)]
extern "C" fn url_has_same_host(target: *const Url, source: *const Url) -> u8 {
	let target = unsafe { &*target };
	let source = unsafe { &*source };

	if target.host() == source.host() { 1 } else { 0 }
}

#[unsafe(no_mangle)]
extern "C" fn url_file_path(url: *const Url) -> *const c_char {
	let url = unsafe { &*url };

	let path = url.path_segments().and_then(|sp| sp.last());

	match path {
		Some(path) => into_c_string(path),
		None => std::ptr::null(),
	}
}

// # panic
// 直接假定input就是合法url！
#[unsafe(no_mangle)]
extern "C" fn url_unsafe_parse(input: *const c_char) -> *const Url {
	let url = unsafe { unsafe_str(input) };
	let url = Url::parse(url).expect(&format!("{url}不是有效的URL！"));
	into_ptr!(url)
}
