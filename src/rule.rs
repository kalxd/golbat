use std::ffi::{c_char, CStr};

#[macro_export]
macro_rules! into_ptr {
	($e:expr) => {
		Box::into_raw(Box::new($e))
	};
}

pub(crate) unsafe fn drop_ptr<T>(ptr: *mut T) {
	let v = unsafe { Box::from_raw(ptr) };
	drop(v);
}

pub(crate) unsafe fn unsafe_str<'a>(ptr: *const c_char) -> &'a str {
	let cstr = unsafe { CStr::from_ptr(ptr) };
	cstr.to_str().expect("不是合法的C字符串")
}
