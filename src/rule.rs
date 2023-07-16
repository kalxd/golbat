use std::ffi::{c_char, CStr};

#[macro_export]
macro_rules! into_ptr {
	($e:expr) => {
		Box::into_raw(Box::new($e))
	};
}

pub(crate) unsafe fn drop_ptr<T>(ptr: *mut T) {
	let v = Box::from_raw(ptr);
	drop(v);
}

pub(crate) unsafe fn unsafe_str<'a>(ptr: *const c_char) -> &'a str {
	let cstr = CStr::from_ptr(ptr);
	cstr.to_str().expect("不是合法的C字符串")
}

pub(crate) unsafe fn take_iter_first<T: Iterator>(ptr: *mut T) -> *const T::Item {
	let mut iter = Box::from_raw(ptr);
	match iter.next() {
		Some(v) => into_ptr!(v),
		None => std::ptr::null(),
	}
}
